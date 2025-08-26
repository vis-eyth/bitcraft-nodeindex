use std::io::{stdout, Write};
use std::net::SocketAddr;
use std::sync::Arc;

mod enums;
use enums::Enum;
mod glue;
use glue::{Config, Configurable, channel_1, channel_2};
mod queue_sub;
use queue_sub::{QueueSub, WithQueueSub};
mod resource;
use resource::RESOURCES;

use bindings::{sdk::{DbContext, Table, TableWithPrimaryKey}, region::*};
use axum::{Router, Json, routing::get, http::StatusCode, extract::{Path, State}};
use axum::http::{HeaderValue, Method};
use intmap::IntMap;
use serde_json::Value;
use tokio::net::TcpListener;
use tokio::sync::{oneshot, RwLock};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};

type NodeMap = RwLock<IntMap<u64, [i32; 2]>>;
struct AppState { pub resource: IntMap<i32, NodeMap>, pub enemy: IntMap<i32, NodeMap> }

enum Message {
    Disconnect,

    ResourceInsert { id: u64, res: i32, x: i32, z: i32 },
    ResourceDelete { id: u64, res: i32 },

    EnemyInsert { id: u64, mob: i32, x: i32, z: i32 },
    EnemyDelete { id: u64, mob: i32 },
}

impl Message {
    pub fn resource_insert(res: &ResourceState, loc: &LocationState) -> Self {
        Self::ResourceInsert { id: res.entity_id, res: res.resource_id, x: loc.x, z: loc.z }
    }

    pub fn resource_delete(res: &ResourceState) -> Self {
        Self::ResourceDelete { id: res.entity_id, res: res.resource_id }
    }

    pub fn enemy_insert(mob: &EnemyState, loc: &MobileEntityState) -> Self {
        Self::EnemyInsert { id: mob.entity_id, mob: mob.enemy_type as i32, x: loc.location_x, z: loc.location_z }
    }

    pub fn enemy_delete(mob: &EnemyState) -> Self {
        Self::EnemyDelete { id: mob.entity_id, mob: mob.enemy_type as i32 }
    }
}

#[tokio::main]
async fn main() {
    let config = Config::from_env()
        .or(Config::from("config.json"))
        .expect("failed to load config!");

    if config.is_empty() {
        eprintln!("please fill out the configuration file (config.json) or use env vars!");
        return;
    }

    let mut sub = QueueSub::new()
        .on_error(|_, err| eprintln!("\nsubscription error: {:?}", err))
        .on_success(|| println!("\nactive!"))
        .on_group(|group| { print!("\n{}", group); stdout().flush().unwrap(); })
        .on_tick(|| { print!("."); stdout().flush().unwrap(); });

    sub.push_group(String::from("resources:"));
    let mut tier = u8::MAX;
    for res in RESOURCES {
        if tier != res.tier {
            sub.push_group(format!("  tier {:>2} ", res.tier));
            tier = res.tier;
        }

        sub.push_query(move || vec![
            format!("SELECT res.* FROM resource_state res JOIN location_state loc ON res.entity_id = loc.entity_id WHERE res.resource_id = {};", res.id),
            format!("SELECT loc.* FROM location_state loc JOIN resource_state res ON loc.entity_id = res.entity_id WHERE res.resource_id = {};", res.id),
        ]);
    }

    sub.push_group(String::from("enemies: "));
    sub.push_query(move || vec![
        String::from("SELECT mob.* FROM enemy_state mob JOIN mobile_entity_state loc ON mob.entity_id = loc.entity_id;"),
        String::from("SELECT loc.* FROM mobile_entity_state loc JOIN enemy_state mob ON loc.entity_id = mob.entity_id;"),
    ]);

    let (tx, rx) = unbounded_channel();
    let con = DbConnection::builder()
        .configure(&config)
        .on_connect(|ctx, _, _| { eprintln!("connected!"); ctx.subscribe(sub); })
        .on_disconnect(|_, _| eprintln!("disconnected!"))
        .build()
        .unwrap();

    con.db.resource_state().on_insert(channel_1(tx.clone(), on_resource_insert));
    con.db.resource_state().on_delete(channel_1(tx.clone(), on_resource_delete));

    con.db.enemy_state().on_insert(channel_1(tx.clone(), on_enemy_insert));
    con.db.enemy_state().on_delete(channel_1(tx.clone(), on_enemy_delete));

    con.db.mobile_entity_state().on_update(channel_2(tx.clone(), on_enemy_move));

    let map = init_state();
    let (tx_sig, rx_sig) = oneshot::channel();

    let mut producer = Box::pin(con.run_async());
    let consumer = tokio::spawn(consume(rx, map.clone()));
    let server = tokio::spawn(server(rx_sig, config, map.clone()));

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            con.disconnect().unwrap();
            producer.await.unwrap();

            tx.send(Message::Disconnect).unwrap();
            tx_sig.send(()).unwrap();

            consumer.await.unwrap();
            server.await.unwrap();
        },
        _ = &mut producer => {
            println!("server disconnect!");

            tx.send(Message::Disconnect).unwrap();
            tx_sig.send(()).unwrap();

            consumer.await.unwrap();
            server.await.unwrap();
        },
    }
}

fn on_resource_insert(ctx: &EventContext, row: &ResourceState, tx: &UnboundedSender<Message>) {
    let Some(loc) = ctx.db.location_state().entity_id().find(&row.entity_id) else {
        eprintln!("no location found for resource: {}", row.entity_id);
        return;
    };

    tx.send(Message::resource_insert(row, &loc)).unwrap()
}

fn on_resource_delete(_: &EventContext, row: &ResourceState, tx: &UnboundedSender<Message>) {
    tx.send(Message::resource_delete(row)).unwrap()
}

fn on_enemy_insert(ctx: &EventContext, row: &EnemyState, tx: &UnboundedSender<Message>) {
    let Some(loc) = ctx.db.mobile_entity_state().entity_id().find(&row.entity_id) else {
        eprintln!("no location found for enemy: {}", row.entity_id);
        return;
    };

    tx.send(Message::enemy_insert(row, &loc)).unwrap()
}

fn on_enemy_delete(_: &EventContext, row: &EnemyState, tx: &UnboundedSender<Message>) {
    tx.send(Message::enemy_delete(row)).unwrap()
}

fn on_enemy_move(ctx: &EventContext, _: &MobileEntityState, new: &MobileEntityState, tx: &UnboundedSender<Message>) {
    let Some(mob) = ctx.db.enemy_state().entity_id().find(&new.entity_id) else {
        eprintln!("no enemy found for location: {}", new.entity_id);
        return;
    };

    tx.send(Message::enemy_insert(&mob, new)).unwrap()
}

fn init_state() -> Arc<AppState> {
    let mut state = AppState { resource: IntMap::new(), enemy: IntMap::new() };

    for res in RESOURCES {
        state.resource.insert(res.id, RwLock::new(IntMap::new()));
    }
    for mob in EnemyType::values() {
        state.enemy.insert(*mob as i32, RwLock::new(IntMap::new()));
    }

    Arc::new(state)
}

async fn consume(mut rx: UnboundedReceiver<Message>, state: Arc<AppState>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            Message::Disconnect => { break; }

            Message::ResourceInsert { id, res, x, z } => {
                state.resource
                    .get(res)
                    .expect("received insert for untracked resource")
                    .write()
                    .await
                    .insert(id, [x, z]);
            }

            Message::ResourceDelete { id, res } => {
                state.resource
                    .get(res)
                    .expect("received delete for untracked resource")
                    .write()
                    .await
                    .remove(id);
            }

            Message::EnemyInsert { id, mob, x, z } => {
                state.enemy
                    .get(mob)
                    .expect("received insert for untracked enemy")
                    .write()
                    .await
                    .insert(id, [x, z]);
            }

            Message::EnemyDelete { id, mob } => {
                state.enemy
                    .get(mob)
                    .expect("received delete for untracked enemy")
                    .write()
                    .await
                    .remove(id);
            }
        }
    }
}

async fn server(rx: oneshot::Receiver<()>, config: Config, state: Arc<AppState>) {
    let mut app = Router::new()
        .route("/resource/{id}", get(route_resource_id))
        .route("/enemy/{id}", get(route_enemy_id))
        .layer(CompressionLayer::new().gzip(true).zstd(true))
        .with_state(state);

    if !config.cors_origin().is_empty() {
        let cors = CorsLayer::new()
            .allow_origin([HeaderValue::from_str(config.cors_origin()).unwrap()])
            .allow_methods([Method::GET, Method::OPTIONS])
            .allow_headers(Any);

        app = app.layer(cors);
    }

    let addr: SocketAddr = config.socket_addr().parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("server listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(async { rx.await.unwrap(); })
        .await
        .unwrap();
}

async fn route_resource_id(
    Path(id): Path<i32>,
    state: State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let Some(nodes) = state.resource.get(id) else {
        return Err((StatusCode::NOT_FOUND, format!("Resource ID not found: {}", id)))
    };
    let nodes = nodes.read().await;

    Ok(Json(serde_json::json!({
        "type": "FeatureCollection",
        "features": [{
            "type": "Feature",
            "properties": { "makeCanvas": "10" },
            "geometry": { "type": "MultiPoint", "coordinates": nodes.values().collect::<Vec<_>>() }
        }]
    })))
}

async fn route_enemy_id(
    Path(id): Path<i32>,
    state: State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let Some(nodes) = state.enemy.get(id) else {
        return Err((StatusCode::NOT_FOUND, format!("Enemy ID not found: {}", id)))
    };
    let nodes = nodes.read().await
        .values()
        .map(|row| row.map(|e| e as f64 / 1_000f64))
        .collect::<Vec<_>>();

    Ok(Json(serde_json::json!({
        "type": "FeatureCollection",
        "features": [{
            "type": "Feature",
            "properties": { "makeCanvas": "10" },
            "geometry": { "type": "MultiPoint", "coordinates": nodes }
        }]
    })))
}