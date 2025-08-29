use std::{net::SocketAddr, path::Path, sync::Arc};
use bindings::sdk::{DbConnectionBuilder, __codegen::SpacetimeModule};
use anyhow::{anyhow, Result};
use intmap::IntMap;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use tokio::sync::RwLock;

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

pub struct EntityGroup {
    pub nodes: RwLock<IntMap<u64, [i32; 2]>>,
    pub properties: Value,
}

pub struct AppState {
    pub resource: IntMap<i32, EntityGroup>,
    pub enemy: IntMap<i32, EntityGroup>,
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
            resource: IntMap::with_capacity(self.resources.len()),
            enemy: IntMap::with_capacity(self.enemies.len()),
        };

        for Entity { id, name: _, properties } in self.resources {
            state.resource.insert(id, EntityGroup { nodes: RwLock::new(IntMap::new()), properties });
        }
        for Entity { id, name: _, properties } in self.enemies {
            state.enemy.insert(id, EntityGroup { nodes: RwLock::new(IntMap::new()), properties });
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

