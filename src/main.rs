mod channels;
mod config;
mod subscription;
use crate::{channels::*, config::*, subscription::*};

use std::io::{stdout, Write};
use std::sync::Arc;
use bindings::{sdk::DbContext, region::*};
use axum::{
    Router, Json, 
    routing::get, 
    http::StatusCode, 
    extract::{Path, State},
    response::{Sse, sse::Event}
};
use axum::http::{Method};
use serde_json::Value;
use tokio::net::TcpListener;
use tokio::sync::{oneshot, mpsc::{unbounded_channel, UnboundedReceiver}, broadcast};
use tokio_stream::{wrappers::BroadcastStream, StreamExt as _};
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use futures_util::stream::Stream;
use chrono;

enum Message {
    Disconnect,

    ResourceInsert { id: u64, res: i32, x: i32, z: i32 },
    ResourceDelete { id: u64, res: i32 },

    EnemyInsert { id: u64, mob: i32, x: i32, z: i32 },
    EnemyDelete { id: u64, mob: i32 },
}

#[derive(Clone)]
struct SseEvent {
    message: String,
}

struct AppStateWithSse {
    app_state: Arc<AppState>,
    sse_tx: broadcast::Sender<SseEvent>,
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
    let config = match AppConfig::from("config.json") {
        Ok(config) => config,
        Err(err) => {
            eprintln!("could not set configuration:");
            eprintln!("  {}", err);
            eprintln!("please check the configuration file (config.json) and your env vars!");
            return;
        }
    };

    let sub = QueueSub::new().on_success(|| {
        println!("\nactive!");
    }).on_error(|ctx, err| {
        println!("\nsubscription error: {:?}", err);
        ctx.disconnect().unwrap();
    }).on_group(|group| {
        print!("\n{}", group);
        stdout().flush().unwrap();
    }).on_tick(|| {
        print!(".");
        stdout().flush().unwrap();
    });

    let (state, db_config, sub, server_config) = config.build(sub);

    // Create SSE broadcast channel
    let (sse_tx, _) = broadcast::channel(1000);
    let app_state_with_sse = AppStateWithSse {
        app_state: state.clone(),
        sse_tx: sse_tx.clone(),
    };

    let (tx, rx) = unbounded_channel();
    let con = DbConnection::builder()
        .configure(&db_config)
        .on_connect(|ctx, _, _| { eprintln!("connected!"); ctx.subscribe(sub); })
        .on_disconnect(|_, _| eprintln!("disconnected!"))
        .with_light_mode(true)
        .build()
        .unwrap();

    con.db.resource_state().on_insert_send(&tx, |ctx, row|
        ctx.db.location_state()
            .entity_id()
            .find(&row.entity_id)
            .map(|loc| Message::resource_insert(row, &loc))
    );
    con.db.resource_state().on_delete_send(&tx, |_, row|
        Some(Message::resource_delete(row))
    );

    con.db.enemy_state().on_insert_send(&tx, |ctx, row|
        ctx.db.mobile_entity_state()
            .entity_id()
            .find(&row.entity_id)
            .map(|loc| Message::enemy_insert(row, &loc))
    );
    con.db.enemy_state().on_delete_send(&tx, |_, row|
        Some(Message::enemy_delete(row))
    );

    con.db.mobile_entity_state().on_update_send(&tx, |ctx, _, new|
        ctx.db.enemy_state()
            .entity_id()
            .find(&new.entity_id)
            .map(|mob| Message::enemy_insert(&mob, new))
    );

    let (tx_sig, rx_sig) = oneshot::channel();

    let mut producer = Box::pin(con.run_async());
    let consumer = tokio::spawn(consume(rx, state.clone(), sse_tx.clone()));
    let server = tokio::spawn(server(rx_sig, server_config, Arc::new(app_state_with_sse)));

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

async fn consume(mut rx: UnboundedReceiver<Message>, state: Arc<AppState>, sse_tx: broadcast::Sender<SseEvent>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            Message::Disconnect => { break; }

            Message::ResourceInsert { id, res, x, z } => {
                if let Some(resource) = state.resource.get(res) {
                    resource.nodes.write().await.insert(id, [x, z]);
                    // Send SSE event
                    let _ = sse_tx.send(SseEvent {
                        message: format!("insert:{}", res),
                    });
                }
            }
            Message::ResourceDelete { id, res } => {
                if let Some(resource) = state.resource.get(res) {
                    resource.nodes.write().await.remove(id);
                    // Send SSE event
                    let _ = sse_tx.send(SseEvent {
                        message: format!("delete:{}", res),
                    });
                }
            }
            Message::EnemyInsert { id, mob, x, z } => {
                if let Some(enemy) = state.enemy.get(mob) {
                    enemy.nodes.write().await.insert(id, [x, z]);
                    // Send SSE event for enemy insert
                    let _ = sse_tx.send(SseEvent {
                        message: format!("enemy_insert:{}", mob),
                    });
                }
            }
            Message::EnemyDelete { id, mob } => {
                if let Some(enemy) = state.enemy.get(mob) {
                    enemy.nodes.write().await.remove(id);
                    // Send SSE event for enemy delete
                    let _ = sse_tx.send(SseEvent {
                        message: format!("enemy_delete:{}", mob),
                    });
                }
            }
        }
    }
}

async fn server(rx: oneshot::Receiver<()>, config: ServerConfig, state: Arc<AppStateWithSse>) {
    let mut app = Router::new()
        .route("/resource/{id}", get(route_resource_id))
        .route("/enemy/{id}", get(route_enemy_id))
        .route("/events", get(route_sse_events))
        .route("/health", get(route_health))
        .route("/resources", get(route_resources))
        .route("/enemies", get(route_enemies))
        .layer(CompressionLayer::new().gzip(true).zstd(true))
        .with_state(state);

    if !config.cors_origin.is_empty() {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::OPTIONS])
            .allow_headers(Any);

        app = app.layer(cors);
    }

    let addr = config.socket_addr;
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("server listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(async { rx.await.unwrap(); })
        .await
        .unwrap();
}

async fn route_resources(
    state: State<Arc<AppStateWithSse>>,
) -> Json<Value> {
    Json(serde_json::json!(state.app_state.resources_list))
}

async fn route_enemies(
    state: State<Arc<AppStateWithSse>>,
) -> Json<Value> {
    Json(serde_json::json!(state.app_state.enemies_list))
}

async fn route_health() -> Json<Value> {
    Json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn route_sse_events(
    state: State<Arc<AppStateWithSse>>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let rx = state.sse_tx.subscribe();
    let stream = BroadcastStream::new(rx)
        .map(|msg| {
            match msg {
                Ok(sse_event) => {
                    Ok(Event::default().data(sse_event.message))
                }
                Err(_) => {
                    // Handle lagged messages by sending a reconnect event
                    Ok(Event::default().event("reconnect").data(""))
                }
            }
        });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(std::time::Duration::from_secs(30))
            .text("keep-alive"),
    )
}

async fn route_resource_id(
    Path(id): Path<i32>,
    state: State<Arc<AppStateWithSse>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let Some(resource) = state.app_state.resource.get(id) else {
        return Err((StatusCode::NOT_FOUND, format!("Resource ID not found: {}", id)))
    };
    let nodes = resource.nodes.read().await;

    Ok(Json(serde_json::json!({
        "type": "FeatureCollection",
        "features": [{
            "type": "Feature",
            "properties": resource.properties,
            "geometry": { "type": "MultiPoint", "coordinates": nodes.values().collect::<Vec<_>>() }
        }]
    })))
}

async fn route_enemy_id(
    Path(id): Path<i32>,
    state: State<Arc<AppStateWithSse>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let Some(enemy) = state.app_state.enemy.get(id) else {
        return Err((StatusCode::NOT_FOUND, format!("Enemy ID not found: {}", id)))
    };
    let nodes = enemy.nodes.read().await
        .values()
        .map(|row| row.map(|e| e as f64 / 1_000f64))
        .collect::<Vec<_>>();

    Ok(Json(serde_json::json!({
        "type": "FeatureCollection",
        "features": [{
            "type": "Feature",
            "properties": enemy.properties,
            "geometry": { "type": "MultiPoint", "coordinates": nodes }
        }]
    })))
}