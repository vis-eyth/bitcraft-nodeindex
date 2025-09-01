use std::{net::SocketAddr, path::Path, sync::Arc};
use std::mem::replace;
use std::sync::Mutex as SyncMutex;
use bindings::sdk::{DbConnectionBuilder, __codegen::SpacetimeModule};
use anyhow::{anyhow, Result};
use hashbrown::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use tokio::sync::{oneshot, RwLock};

fn default_properties() -> Value { json!({ "makeCanvas": "10" }) }
fn default_socket_addr() -> SocketAddr { SocketAddr::from(([0, 0, 0, 0], 3000)) }


#[derive(Serialize, Deserialize)]
pub struct Entity {
    pub id: i32,
    #[serde(default = "Default::default")]
    pub name: String,
    #[serde(default = "default_properties")]
    pub properties: Value,
}

#[derive(Serialize, Deserialize)]
pub struct DbConfig {
    #[serde(default = "Default::default")]
    pub region: u8,
    #[serde(default = "Default::default")]
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_socket_addr")]
    pub socket_addr: SocketAddr,
    #[serde(default = "Default::default")]
    pub cors_origin: String,
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub db: DbConfig,
    pub server: ServerConfig,
    pub resources: Vec<Entity>,
    pub enemies: Vec<Entity>,
}

#[derive(PartialEq)]
pub enum DataState {
    ACTIVE, STALE,
}

pub struct EntityState {
    pub nodes: HashMap<u64, [i32; 2]>,
    pub state: DataState,
    pub properties: Value,
}

pub enum ConnectionState {
    CONNECTING(i32), SYNCHRONIZING, ACTIVE, DISCONNECTED(oneshot::Sender<()>),
}

pub struct AppState {
    pub state: SyncMutex<ConnectionState>,
    pub resource: HashMap<i32, RwLock<EntityState>>,
    pub enemy: HashMap<i32, RwLock<EntityState>>,
}

impl AppState {
    pub fn set_state(&self, new: ConnectionState) {
        let mut state = self.state.lock().unwrap();
        *state = new;
    }

    pub fn on_disconnect(&self) -> Option<oneshot::Receiver<()>> {
        let mut state = self.state.lock().unwrap();
        match *state {
            ConnectionState::CONNECTING(1) => {
                let (tx, rx) = oneshot::channel();
                *state = ConnectionState::DISCONNECTED(tx);
                Some(rx)
            }
            ConnectionState::CONNECTING(n) => {
                *state = ConnectionState::CONNECTING(n - 1);
                None
            }
            _ => {
                *state = ConnectionState::CONNECTING(5);
                None
            }
        }
    }

    pub fn on_reconnect(&self) -> Option<oneshot::Sender<()>> {
        let mut state = self.state.lock().unwrap();

        let prev: ConnectionState = replace(&mut state, ConnectionState::CONNECTING(5));
        if let ConnectionState::DISCONNECTED(tx) = prev {
            return Some(tx)
        }

        let _: ConnectionState = replace(&mut state, prev);
        None
    }
}


impl AppConfig {
    pub fn from(path: &str) -> Result<Self> {
        let path = Path::new(path);
        let content = std::fs::read(path)?;
        let mut config: AppConfig = serde_json::from_slice(&content)?;

        if let Ok(token) = std::env::var("TOKEN") {
            config.db.token = token;
        }
        if let Ok(region) = std::env::var("REGION") {
            config.db.region = region.parse()
                .map_err(|_| anyhow!("invalid region, needs to be a number (1-9)"))?;
        }
        if let Ok(socket_addr) = std::env::var("SOCKET_ADDR") {
            config.server.socket_addr = socket_addr.parse()?;
        }
        if let Ok(cors_origin) = std::env::var("CORS_ORIGIN") {
            config.server.cors_origin = cors_origin;
        }

        if config.db.token.is_empty() {
            return Err(anyhow!("token is empty"));
        }
        if config.server.cors_origin == "*" {
            return Err(anyhow!("CORS origin may not be any, unset to disable"));
        }

        Ok(config)
    }

    pub fn build(self) -> (Arc<AppState>, DbConfig, ServerConfig) {
        let mut state = AppState {
            state: SyncMutex::new(ConnectionState::CONNECTING(5)),
            resource: HashMap::with_capacity(self.resources.len()),
            enemy: HashMap::with_capacity(self.enemies.len()),
        };

        for Entity { id, name: _, properties } in self.resources {
            state.resource.insert(id, RwLock::new(EntityState {
                nodes: HashMap::new(), state: DataState::STALE, properties
            }));
        }
        for Entity { id, name: _, properties } in self.enemies {
            state.enemy.insert(id, RwLock::new(EntityState {
                nodes: HashMap::new(), state: DataState::STALE, properties
            }));
        }

        (Arc::new(state), self.db, self.server)
    }
}


pub trait WithDbConfig { fn configure(self, config: &DbConfig) -> Self; }
impl<M: SpacetimeModule> WithDbConfig for DbConnectionBuilder<M>
{
    fn configure(self, config: &DbConfig) -> Self {
        self.with_uri("https://bitcraft-early-access.spacetimedb.com")
            .with_module_name(format!("bitcraft-{}", config.region))
            .with_token(Some(&config.token))
    }
}

