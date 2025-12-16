use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub crm_gateway_port: u16,
    pub from: String,
    pub user_stat_url: String,
    pub metadata_url: String,
    pub notify_url: String,
    pub pg_url: String,
}

#[derive(Deserialize)]
pub struct AuthConfig {
    pub verify_key: String,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let config_file = cargo_dir.join("../config.yaml");

        let reader = match std::fs::File::open(config_file) {
            Ok(reader) => reader,
            Err(e) => {
                return Err(anyhow::anyhow!("open config file failed: {:?}", e));
            }
        };

        let app_config = serde_yaml::from_reader(reader)?;

        Ok(app_config)
    }
}
