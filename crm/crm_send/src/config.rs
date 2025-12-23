use std::{fs::File, path::PathBuf};

use anyhow::Result;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub notify_port: u16,
    pub pg_url: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let config_path = cargo_dir.join("../config.yaml");

        let reader = match File::open(config_path) {
            Ok(reader) => reader,
            Err(e) => return Err(anyhow::anyhow!("Failed to open config file: {}", e)),
        };

        let ret = match serde_yaml::from_reader(reader) {
            Ok(ret) => ret,
            Err(e) => return Err(anyhow::anyhow!("Failed to parse config file: {}", e)),
        };

        Ok(ret)
    }
}
