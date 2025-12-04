use std::{fs::File, path::PathBuf};

use anyhow::Result;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub sse_port: u16,
    pub pg_url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AuthConfig {
    pub verify_key: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let config_dir = cargo_dir.join("../config.yaml");

        let reader = match File::open(config_dir) {
            Ok(reader) => reader,
            Err(e) => return Err(anyhow::anyhow!("Failed to open config file, error: {}", e)),
        };

        let ret = match serde_yaml::from_reader(reader) {
            Ok(ret) => ret,
            Err(e) => return Err(anyhow::anyhow!("Failed to parse config file, error: {}", e)),
        };

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() -> anyhow::Result<()> {
        let config = AppConfig::load()?;
        println!("{:?}", config);

        Ok(())
    }
}
