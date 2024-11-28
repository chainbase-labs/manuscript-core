use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::sync::Once;
use lazy_static::lazy_static;

static INIT: Once = Once::new();

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub base_url: String,
    pub endpoints: ApiEndpoints,
}

#[derive(Debug, Deserialize)]
pub struct ApiEndpoints {
    pub chains: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub api: ApiConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        
        if let Ok(_) = s.merge(File::with_name("config/default")) {
        } else {
            if let Ok(exe_path) = std::env::current_exe() {
                if let Some(exe_dir) = exe_path.parent() {
                    let config_path = exe_dir.join("config/default");
                    s.merge(File::with_name(config_path.to_str().unwrap()))?;
                }
            }
        }

        s.try_deserialize()
    }

    pub fn get_chains_url() -> String {
        lazy_static! {
            static ref SETTINGS: Settings = Settings::new().expect("Failed to load settings");
        }
        format!("{}{}", SETTINGS.api.base_url, SETTINGS.api.endpoints.chains)
    }
} 