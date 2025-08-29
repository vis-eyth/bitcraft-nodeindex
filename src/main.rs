mod config;
mod subscription;
mod database;
use crate::{config::*, database::*, subscription::*};

use std::sync::Arc;
use bindings::{sdk::DbContext, region::*, ext::ctx::*};
use anyhow::Result;
use axum::{Router, Json, routing::get, http::StatusCode, extract::{Path, State}};
use axum::http::{HeaderValue, Method};
use serde_json::Value;
use tokio::net::TcpListener;
use tokio::sync::{oneshot, mpsc::unbounded_channel};
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

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

    let mut queries = Vec::new();
    if !state.enemy.is_empty() { queries.push(Query::ENEMY) }
    for id in state.resource.keys() { queries.push(Query::RESOURCE(id)) }

    let sub = QueueSub::with(queries)
        .on_success(|| {
            info!("active!");
        })
        .on_error(|ctx, err| {
            error!("db error while subscribing: {:?}", err);
            ctx.disconnect().unwrap();
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
            tx_sig.send(()).unwrap();
        })
        .with_light_mode(true)
        .with_channel(tx)
        .build()
        .unwrap();

    tokio::spawn(consume(rx, state.clone()));

    let (con, server) = tokio::join!(
        tokio::spawn(con.run_until(tokio::signal::ctrl_c())),
        tokio::spawn(server(rx_sig, server_config, state.clone())),
    );

    if let Ok(Err(e)) = con { error!("db error: {:#}", e); }
    if let Ok(Err(e)) = server { error!("server error: {:#}", e); }
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
        .with_graceful_shutdown(async { rx.await.unwrap(); })
        .await?;

    Ok(())
}

async fn route_resource_id(
    Path(id): Path<i32>,
    state: State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let Some(resource) = state.resource.get(id) else {
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
    state: State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let Some(enemy) = state.enemy.get(id) else {
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