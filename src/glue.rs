use anyhow::Result;
use std::path::Path;
use serde;
use bindings::sdk::{DbConnectionBuilder, __codegen::SpacetimeModule};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    socket_addr: String,
    cors_origin: String,
    cluster_url: String,
    region:      String,
    token:       String,
}

impl Config {
    fn new() -> Self {
        Self {
            socket_addr: String::new(),
            cors_origin: String::new(),
            cluster_url: String::new(),
            region: String::new(),
            token: String::new(),
        }
    }

    pub fn from_env() -> Result<Self> {
        let socket_addr = std::env::var("SOCKET_ADDR")
            .unwrap_or(String::from("0.0.0.0:3000"));
        let cors_origin = std::env::var("CORS_ORIGIN")
            .unwrap_or(String::new());
        let cluster_url = std::env::var("CLUSTER_URL")
            .unwrap_or(String::from("https://bitcraft-early-access.spacetimedb.com"));

        let region = std::env::var("REGION")?;
        let token = std::env::var("TOKEN")?;

        Ok(Self { socket_addr, cors_origin, cluster_url, region, token })
    }

    pub fn from(path: &str) -> Result<Self> {
        let path = Path::new(path);
        if !path.exists() {
            let config = Config::new();
            let content = serde_json::to_string_pretty(&config)?;
            std::fs::write(path, content)?;
            Ok(config)
        } else {
            let content = std::fs::read(path)?;
            let config = serde_json::from_slice(&content)?;
            Ok(config)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cluster_url.is_empty() || self.region.is_empty() || self.token.is_empty()
    }
    
    pub fn socket_addr(&self) -> &str { &self.socket_addr }
    pub fn cors_origin(&self) -> &str { &self.cors_origin }
}

pub trait Configurable<MOD>
where MOD: SpacetimeModule
{
    fn configure(self, config: &Config) -> Self;
}

impl <MOD> Configurable<MOD> for DbConnectionBuilder<MOD>
where MOD: SpacetimeModule
{
    fn configure(self, config: &Config) -> Self {
        self.with_uri(&config.cluster_url)
            .with_module_name(&config.region)
            .with_token(Some(&config.token))
    }
}