mod channels;
mod config;
mod subscription;
use crate::{channels::*, config::*, subscription::*};

use std::io::{stdout, Write};
use std::sync::Arc;
use std::collections::HashSet;
use tokio::sync::RwLock as TokioRwLock;
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

    PlayerInsert { id: u64, char: i32, x: i32, z: i32 },
    PlayerDelete { id: u64, char: i32 },
    
    PlayerSignIn { id: u64, username: Option<String> },
    PlayerSignOut { id: u64 },
    
    PlayerNameUpdate { id: u64, username: String },
}

#[derive(Clone)]
struct SseEvent {
    message: String,
}

struct AppStateWithSse {
    app_state: Arc<AppState>,
    sse_tx: broadcast::Sender<SseEvent>,
    signed_in_players: Arc<TokioRwLock<HashSet<u64>>>,
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

    pub fn player_insert(entity_id: u64, loc: &MobileEntityState) -> Self {
        Self::PlayerInsert { id: entity_id, char: 1, x: loc.location_x, z: loc.location_z }
    }

    pub fn player_delete(entity_id: u64) -> Self {
        Self::PlayerDelete { id: entity_id, char: 1 }
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
    let signed_in_players = Arc::new(TokioRwLock::new(HashSet::new()));
    let app_state_with_sse = AppStateWithSse {
        app_state: state.clone(),
        sse_tx: sse_tx.clone(),
        signed_in_players: signed_in_players.clone(),
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

    // Player event handlers - track signed-in players using message passing
    con.db.signed_in_player_state().on_insert_send(&tx, |ctx, row| {
        // Look up the player's username when they sign in
        let username = ctx.db.player_username_state()
            .entity_id()
            .find(&row.entity_id)
            .map(|player_username| player_username.username);
        
        Some(Message::PlayerSignIn { id: row.entity_id, username })
    });
    
    con.db.signed_in_player_state().on_delete_send(&tx, |_, row| {
        Some(Message::PlayerSignOut { id: row.entity_id })
    });
    
    // Track player name changes
    con.db.player_username_state().on_insert_send(&tx, |_, row| {
        Some(Message::PlayerNameUpdate { id: row.entity_id, username: row.username.clone() })
    });
    
    con.db.player_username_state().on_update_send(&tx, |_, _, new_row| {
        Some(Message::PlayerNameUpdate { id: new_row.entity_id, username: new_row.username.clone() })
    });
    
    // Comprehensive mobile entity handler to distinguish between enemies and potential players
    con.db.mobile_entity_state().on_update_send(&tx, {
        let tx_clone = tx.clone();
        move |ctx, _, new| {
            if let Some(mob) = ctx.db.enemy_state().entity_id().find(&new.entity_id) {
                // This is an enemy
                Some(Message::enemy_insert(&mob, new))
            } else {
                // This could be a player - we'll filter by signed-in status in consume()
                // Also try to get the username if this is a signed-in player
                if ctx.db.signed_in_player_state().entity_id().find(&new.entity_id).is_some() {
                    // This is a signed-in player, get their username
                    let username = ctx.db.player_username_state()
                        .entity_id()
                        .find(&new.entity_id)
                        .map(|player_username| player_username.username)
                        .unwrap_or_else(|| format!("Player_{}", new.entity_id));
                    
                    // Send name update first
                    let _ = tx_clone.send(Message::PlayerNameUpdate { id: new.entity_id, username });
                }
                
                Some(Message::player_insert(new.entity_id, &new))
            }
        }
    });

    let (tx_sig, rx_sig) = oneshot::channel();

    let mut producer = Box::pin(con.run_async());
    let consumer = tokio::spawn(consume(rx, state.clone(), sse_tx.clone(), signed_in_players.clone()));
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

async fn consume(
    mut rx: UnboundedReceiver<Message>, 
    state: Arc<AppState>, 
    sse_tx: broadcast::Sender<SseEvent>,
    signed_in_players: Arc<TokioRwLock<HashSet<u64>>>,
) {
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
            Message::PlayerInsert { id, char, x, z } => {
                // Check if this player is actually signed in
                if signed_in_players.read().await.contains(&id) {
                    if let Some(player) = state.player.get(char) {
                        player.nodes.write().await.insert(id, [x, z]);
                        // Send SSE event for player insert with entity ID
                        let _ = sse_tx.send(SseEvent {
                            message: format!("player_insert:{}:{}", char, id),
                        });
                    } else {
                        eprintln!("Warning: Received PlayerInsert for unknown character {}", char);
                    }
                } else {
                    // This is not a signed-in player, ignore the event
                    // (Could be an offline player or other mobile entity)
                }
            }
            Message::PlayerDelete { id, char } => {
                // For deletes, we'll process them regardless of signed-in status
                // since the player might have just signed out
                if let Some(player) = state.player.get(char) {
                    if player.nodes.write().await.remove(id).is_some() {
                        // Send SSE event for player delete with entity ID
                        let _ = sse_tx.send(SseEvent {
                            message: format!("player_delete:{}:{}", char, id),
                        });
                    } else {
                        eprintln!("Warning: Tried to remove non-existent player entity {}", id);
                    }
                } else {
                    eprintln!("Warning: Received PlayerDelete for unknown character {}", char);
                }
            }
            Message::PlayerSignIn { id, username } => {
                signed_in_players.write().await.insert(id);
                
                // If we have a username, store it in all player groups
                if let Some(username) = username {
                    for (_, player_group) in state.player.iter() {
                        player_group.player_names.write().await.insert(id, username.clone());
                    }
                }
            }
            Message::PlayerSignOut { id } => {
                signed_in_players.write().await.remove(&id);
                
                // Remove the player's location data from all player groups
                for (char_id, player_group) in state.player.iter() {
                    if player_group.nodes.write().await.remove(id).is_some() {
                        // Also remove the player name
                        player_group.player_names.write().await.remove(&id);
                        // Send SSE event for player removal with entity ID
                        let _ = sse_tx.send(SseEvent {
                            message: format!("player_delete:{}:{}", char_id, id),
                        });
                    }
                }
            }
            Message::PlayerNameUpdate { id, username } => {
                // Update the player name in all player groups, regardless of whether they have location data yet
                // This ensures names are available when players sign in before moving
                for (_, player_group) in state.player.iter() {
                    // Store the username for this player in all player groups
                    // We'll filter by signed-in status when serving API responses
                    if signed_in_players.read().await.contains(&id) {
                        player_group.player_names.write().await.insert(id, username.clone());
                    }
                }
            }
        }
    }
}

async fn server(rx: oneshot::Receiver<()>, config: ServerConfig, state: Arc<AppStateWithSse>) {
    let mut app = Router::new()
        .route("/resource/{id}", get(route_resource_id))
        .route("/enemy/{id}", get(route_enemy_id))
        .route("/player/{id}", get(route_player_id))
        .route("/events", get(route_sse_events))
        .route("/health", get(route_health))
        .route("/resources", get(route_resources))
        .route("/enemies", get(route_enemies))
        .route("/players", get(route_players))
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
        .with_graceful_shutdown(async { 
            if let Err(e) = rx.await {
                eprintln!("Shutdown signal error: {:?}", e);
            }
        })
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

async fn route_players(
    state: State<Arc<AppStateWithSse>>,
) -> Json<Value> {
    Json(serde_json::json!(state.app_state.players_list))
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
    let nodes = enemy.nodes.read().await;
    
    // Keep MultiPoint format for enemies (unlike players which use individual Point features)
    let coordinates: Vec<[f64; 2]> = nodes
        .values()
        .map(|coords| [coords[0] as f64 / 1_000f64, coords[1] as f64 / 1_000f64])
        .collect();

    Ok(Json(serde_json::json!({
        "type": "FeatureCollection",
        "features": [
            {
                "type": "Feature",
                "properties": enemy.properties,
                "geometry": {
                    "type": "MultiPoint",
                    "coordinates": coordinates
                }
            }
        ]
    })))
}

async fn route_player_id(
    Path(id): Path<i32>,
    state: State<Arc<AppStateWithSse>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let Some(player) = state.app_state.player.get(id) else {
        return Err((StatusCode::NOT_FOUND, format!("Player ID not found: {}", id)))
    };
    let nodes = player.nodes.read().await;
    let player_names = player.player_names.read().await;
    
    // Create individual Point features with entity IDs and player names in properties
    let features: Vec<serde_json::Value> = nodes
        .iter()
        .map(|(entity_id, coords)| {
            let player_name = player_names.get(&entity_id)
                .cloned()
                .unwrap_or_else(|| format!("Player_{}", entity_id));
            
            serde_json::json!({
                "type": "Feature",
                "properties": {
                    "entity_id": entity_id,
                    "player_name": player_name,
                    "makeCanvas": player.properties.get("makeCanvas").unwrap_or(&serde_json::json!("10"))
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [coords[0] as f64 / 1_000f64, coords[1] as f64 / 1_000f64]
                }
            })
        })
        .collect();

    Ok(Json(serde_json::json!({
        "type": "FeatureCollection",
        "features": features
    })))
}