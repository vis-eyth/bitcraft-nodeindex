mod config;
mod subscription;
mod database;
use crate::{config::*, database::*, subscription::*};

use std::sync::Arc;
use bindings::{sdk::DbContext, region::*, ext::ctx::*};
use anyhow::{anyhow, Error, Result};
use axum::{Router, Json, routing::get, http::StatusCode, extract::{Path, State, Query}};
use axum::http::{HeaderValue, Method};
use axum::response::IntoResponse;
use serde::Deserialize;
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
    if !state.enemy.is_empty() { queries.push(Subscription::ENEMY) }
    for id in state.resource.keys() { queries.push(Subscription::RESOURCE(*id)) }

    let (con, server) = tokio::join!(
        spawn(db(db_config, queries, state.clone(), shutdown.clone()), &shutdown),
        spawn(server(server_config, state.clone(), shutdown.clone()), &shutdown),
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

async fn db(config: DbConfig, queries: Vec<Subscription>, state: Arc<AppState>, shutdown: Arc<Mutex<Shutdown>>) -> Result<()> {
    let (tx, rx) = unbounded_channel();
    tokio::spawn(consume(rx, state.clone()));

    let state_active = {
        let state = state.clone();
        move || { state.set_state(ConnectionState::ACTIVE) }
    };
    let state_sync = {
        let state = state.clone();
        move || { state.set_state(ConnectionState::SYNCHRONIZING) }
    };

    loop {
        let state_active = state_active.clone();
        let state_sync = state_sync.clone();

        info!("connecting...");
        let sub = QueueSub::with(queries.clone())
            .on_success(move || {
                info!("active!");
                state_active();
            })
            .on_error(|ctx, err| {
                error!("db error while subscribing: {:?}", err);
                let _ = ctx.disconnect();
            });

        if let Ok(con) = DbConnection::builder()
            .configure(&config)
            .on_connect(move |ctx, _, _| {
                info!("connected!");
                state_sync();
                ctx.subscribe(sub);
            })
            .on_disconnect(move |_, _| {
                info!("disconnected!");
            })
            .with_light_mode(true)
            .with_channel(tx.clone())
            .build()
        {
            let Some(signal) = shutdown.lock().await.register() else { return Ok(()) };
            con.run_until(signal).await?;
        }


        for data in state.resource.values() {
            data.write().await.state = DataState::STALE;
        }
        for data in state.enemy.values() {
            data.write().await.state = DataState::STALE;
        }

        if let Some(barrier) = state.on_disconnect() {
            let Some(signal) = shutdown.lock().await.register() else { return Ok(()) };
            tokio::select! { _ = signal => { return Ok(())}, _ = barrier => {} }
        }
    }
}

async fn server(config: ServerConfig, state: Arc<AppState>, shutdown: Arc<Mutex<Shutdown>>) -> Result<()> {
    let mut app = Router::new()
        .route("/status", get(route_status))
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

    let Some(signal) = shutdown.lock().await.register() else { return Ok(()) };
    axum::serve(listener, app)
        .with_graceful_shutdown(async { let _ = signal.await; })
        .await?;

    Ok(())
}

#[derive(Deserialize)] struct Reconnect { reconnect: Option<bool> }

async fn route_status(State(state): State<Arc<AppState>>, Query(query): Query<Reconnect>) -> impl IntoResponse {
    if query.reconnect.is_some_and(|v| v) && let Some(barrier) = state.on_reconnect() {
        info!("reconnect triggered!");
        let _ = barrier.send(());
    }

    let body = match *state.state.lock().unwrap() {
        ConnectionState::CONNECTING(n) =>
            serde_json::json!({"status": format!("CONNECTING ({})", n)}),
        ConnectionState::SYNCHRONIZING =>
            serde_json::json!({"status": "SYNCHRONIZING"}),
        ConnectionState::ACTIVE =>
            serde_json::json!({"status": "ACTIVE"}),
        ConnectionState::DISCONNECTED(_) =>
            serde_json::json!({"status": "DISCONNECTED"}),
    };
    Json(body)
}

async fn route_resource_id(Path(id): Path<i32>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let Some(resource) = state.resource.get(&id) else {
        return Err((StatusCode::NOT_FOUND, format!("Resource ID not found: {}", id)))
    };
    let data = resource.read().await;
    let nodes = data.nodes
        .values()
        .collect::<Vec<_>>();

    Ok(Json(serde_json::json!({
        "type": "FeatureCollection",
        "features": [{
            "type": "Feature",
            "properties": data.properties,
            "geometry": { "type": "MultiPoint", "coordinates": nodes }
        }]
    })))
}

async fn route_enemy_id(Path(id): Path<i32>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let Some(enemy) = state.enemy.get(&id) else {
        return Err((StatusCode::NOT_FOUND, format!("Enemy ID not found: {}", id)))
    };
    let data = enemy.read().await;
    let nodes = data.nodes
        .values()
        .map(|[x, z]| [*x as f64 / 1_000_f64, *z as f64 / 1_000_f64])
        .collect::<Vec<_>>();

    Ok(Json(serde_json::json!({
        "type": "FeatureCollection",
        "features": [{
            "type": "Feature",
            "properties": data.properties,
            "geometry": { "type": "MultiPoint", "coordinates": nodes }
        }]
    })))
}