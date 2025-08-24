use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use bindings::region::*;

mod resource;
use resource::{RESOURCES, ResourceSubscription};

mod glue;
use glue::{Config, Configurable, with_channel};

use spacetimedb_sdk::{DbContext, Table};
use axum::{Router, Json, routing::get, http::StatusCode, extract::{Path, State}};
use serde_json::Value;
use tokio::net::TcpListener;
use tokio::sync::{oneshot, RwLock};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tower_http::compression::CompressionLayer;

type NodeMap = HashMap<u64, (i32, i32)>;

enum Message {
    Disconnect,
    Insert { id: u64, res: i32, x: i32, z: i32 },
    Delete { id: u64, res: i32 },
}

impl Message {
    pub fn insert(res: &ResourceState, loc: &LocationState) -> Self {
        Self::Insert { id: res.entity_id, res: res.resource_id, x: loc.x, z: loc.z }
    }

    pub fn delete(res: &ResourceState) -> Self {
        Self::Delete { id: res.entity_id, res: res.resource_id }
    }
}

#[tokio::main]
async fn main() {
    let config = Config::from("config.json").expect("failed to load config.json");

    if config.is_empty() {
        eprintln!("please fill out the configuration file (config.json)!");
        return;
    }

    let (tx, rx) = unbounded_channel();
    let con = DbConnection::builder()
        .configure(&config)
        .on_connect(|ctx, _, _| {
            eprintln!("connected!");
            ctx.subscription_builder().all_resources(
                |_, err| eprintln!("subscription error: {:?}", err),
                |_| println!("active!"));
        })
        .on_disconnect(|_, _| eprintln!("disconnected!"))
        .build()
        .unwrap();

    con.db.resource_state().on_insert(with_channel(tx.clone(), on_insert));
    con.db.resource_state().on_delete(with_channel(tx.clone(), on_delete));

    let map = init_shared_map();
    let (tx_sig, rx_sig) = oneshot::channel();

    let mut producer = Box::pin(con.run_async());
    let consumer = tokio::spawn(consume(rx, map.clone()));
    let server = tokio::spawn(server(rx_sig, map.clone()));

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

fn on_insert(ctx: &EventContext, row: &ResourceState, tx: &UnboundedSender<Message>) {
    let loc = ctx.db.location_state()
        .entity_id()
        .find(&row.entity_id);

    if let Some(loc) = loc {
        tx.send(Message::insert(row, &loc)).unwrap()
    } else {
        eprintln!("no location found for resource: {}", row.entity_id);
    }
}

fn on_delete(_: &EventContext, row: &ResourceState, tx: &UnboundedSender<Message>) {
    tx.send(Message::delete(row)).unwrap()
}

fn init_shared_map() -> Arc<HashMap<i32, RwLock<NodeMap>>> {
    let mut map = HashMap::new();
    for res in RESOURCES {
        map.insert(res.id, RwLock::new(HashMap::new()));
    }

    Arc::new(map)
}

async fn consume(mut rx: UnboundedReceiver<Message>, map: Arc<HashMap<i32, RwLock<NodeMap>>>) {
    while let Some(msg) = rx.recv().await {
        if let Message::Disconnect = &msg { break }

        if let Message::Insert{ id, res, x, z } = msg {
            map.get(&res)
                .expect("received insert for untracked resource")
                .write()
                .await
                .insert(id, (x, z));
        }

        if let Message::Delete { id, res } = msg {
            map.get(&res)
                .expect("received delete for untracked resource")
                .write()
                .await
                .remove(&id);
        }
    }
}

async fn server(rx: oneshot::Receiver<()>, map: Arc<HashMap<i32, RwLock<NodeMap>>>) {
    let app = Router::new()
        .route("/resource/{id}", get(route_resource_id))
        .layer(CompressionLayer::new().gzip(true).zstd(true))
        .with_state(map);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("server listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(async { rx.await.unwrap(); })
        .await
        .unwrap();
}

async fn route_resource_id(
    Path(id): Path<i32>,
    state: State<Arc<HashMap<i32, RwLock<NodeMap>>>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let nodes =
        if let Some(nodes) = state.get(&id) { nodes }
        else { return Err((StatusCode::NOT_FOUND, format!("Resource ID not found: {}", id))) };

    let nodes: Vec<Vec<i32>> = nodes.read().await
        .values()
        .map(|(x, z)| vec![*x, *z])
        .collect();

    Ok(Json(serde_json::json!({
        "type": "FeatureCollection",
        "features": [{
            "type": "Feature",
            "properties": { "makeCanvas": "10" },
            "geometry": { "type": "MultiPoint", "coordinates": nodes }
        }]
    })))
}