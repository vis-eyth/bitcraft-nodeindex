mod config;
mod subscription;
mod database;
use crate::{config::*, database::*, subscription::*};

use std::sync::Arc;
use bindings::{sdk::DbContext, region::*, ext::ctx::*};
use anyhow::{anyhow, Error, Result};
use axum::{Router, Json, routing::get, http::StatusCode, extract::{Path, State}};
use axum::http::{HeaderValue, Method};
use axum::response::IntoResponse;
use tokio::net::TcpListener;
use tokio::sync::{oneshot, mpsc::unbounded_channel, Mutex};
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

struct Shutdown { triggered: bool, tx: Vec<oneshot::Sender<()>> }
impl Shutdown {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self { triggered: false, tx: Vec::new() }))
    }
    fn register(&mut self) -> Option<oneshot::Receiver<()>> {
        if self.triggered { return None }

        let (tx, rx) = oneshot::channel();
        self.tx.push(tx);
        Some(rx)
    }
    fn trigger(&mut self) {
        self.triggered = true;
        for tx in self.tx.drain(..) { let _ = tx.send(()); }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = match AppConfig::from("config.json") {
        Ok(config) => config,
        Err(err) => {
            error!("could not set config: {:#}", err);
            return;
        }
    };

    let (state, db_config, server_config) = config.build();

    let shutdown = Shutdown::new();
    register_ctrl_c(shutdown.clone());

    let mut queries = Vec::new();
    if !state.enemy.is_empty() { queries.push(Query::ENEMY) }
    for id in state.resource.keys() { queries.push(Query::RESOURCE(*id)) }

    let sub = QueueSub::with(queries)
        .on_success(|| {
            info!("active!");
        })
        .on_error(|ctx, err| {
            error!("db error while subscribing: {:?}", err);
            let _ = ctx.disconnect();
        });

    let (tx, rx) = unbounded_channel();
    let (tx_sig, rx_sig) = oneshot::channel();

    let con = DbConnection::builder()
        .configure(&db_config)
        .on_connect(|ctx, _, _| {
            info!("connected!");
            ctx.subscribe(sub);
        })
        .on_disconnect(move |_, _| {
            info!("disconnected!");
            let _ = tx_sig.send(());
        })
        .with_light_mode(true)
        .with_channel(tx)
        .build()
        .unwrap();

    tokio::spawn(consume(rx, state.clone()));

    let Some(signal_shutdown) = shutdown.lock().await.register() else { return };
    let (con, server) = tokio::join!(
        spawn(con.run_until(signal_shutdown), &shutdown),
        spawn(server(rx_sig, server_config, state.clone()), &shutdown),
    );

    if let Err(e) = con { error!("db error: {:#}", e); }
    if let Err(e) = server { error!("server error: {:#}", e); }
}

fn register_ctrl_c(shutdown: Arc<Mutex<Shutdown>>) {
    tokio::spawn(async move {
        let _ = tokio::signal::ctrl_c().await;
        info!("ctrl-c pressed, shutting down");
        shutdown.lock().await.trigger();
    });
}

async fn spawn<E: Into<Error> + Send + 'static>(
    future: impl Future<Output = Result<(), E>> + Send + 'static,
    shutdown: &Arc<Mutex<Shutdown>>
) -> Result<()> {
    match tokio::spawn(future).await {
        Ok(Ok(v)) => { Ok(v) }
        Ok(Err(e)) => { shutdown.lock().await.trigger(); Err(anyhow!(e)) }
        Err(e) => { shutdown.lock().await.trigger(); Err(anyhow!(e)) }
    }
}

async fn server(rx: oneshot::Receiver<()>, config: ServerConfig, state: Arc<AppState>) -> Result<()> {
    let mut app = Router::new()
        .route("/resource/{id}", get(route_resource_id))
        .route("/enemy/{id}", get(route_enemy_id))
        .layer(CompressionLayer::new().gzip(true).zstd(true))
        .with_state(state);

    if !config.cors_origin.is_empty() {
        let cors = CorsLayer::new()
            .allow_origin([HeaderValue::from_str(&config.cors_origin)?])
            .allow_methods([Method::GET, Method::OPTIONS])
            .allow_headers(Any);

        app = app.layer(cors);
    }

    let addr = config.socket_addr;
    let listener = TcpListener::bind(addr).await?;

    info!("server listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(async { let _ = rx.await; })
        .await?;

    Ok(())
}

async fn route_resource_id(Path(id): Path<i32>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let Some(resource) = state.resource.get(&id) else {
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

async fn route_enemy_id(Path(id): Path<i32>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let Some(enemy) = state.enemy.get(&id) else {
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