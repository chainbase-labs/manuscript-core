use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;
use lazy_static::lazy_static;
use std::path::Path;

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
pub struct AppConfig {
    pub network: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub api: ApiConfig,
    pub app: AppConfig,
}

lazy_static! {
    static ref SETTINGS: Result<Settings, ConfigError> = Settings::new();
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut builder = Config::builder();

        let exe_path = std::env::current_exe()
            .unwrap_or_else(|_| std::path::PathBuf::from("."));
        let exe_dir = exe_path.parent().unwrap_or_else(|| std::path::Path::new("."));

        let config_locations = vec![
            "config/default.yaml".to_string(),
            "../config/default.yaml".to_string(),
            "../../config/default.yaml".to_string(),
            format!("{}/config/default.yaml", exe_dir.display()),
            format!("{}/config/default.yaml",
                std::env::var("OUT_DIR").unwrap_or_else(|_| String::from("target/debug"))),
        ];

        let mut config_loaded = false;
        for location in config_locations {
            let path = Path::new(&location);
            if path.exists() {
                let file = File::with_name(&location).format(FileFormat::Yaml);
                builder = builder.add_source(file);
                config_loaded = true;
                break;
            } else {
                println!("Configuration file not found: {}", location);
            }
        }

        if !config_loaded {
            return Err(ConfigError::NotFound("Could not find config file".into()));
        }

        builder.build()?.try_deserialize()
    }

    pub fn get_chains_url() -> String {
        match &*SETTINGS {
            Ok(settings) => format!("{}{}", settings.api.base_url, settings.api.endpoints.chains),
            Err(e) => {
                eprintln!("Failed to load settings: {}", e);
                String::from("https://api.chainbase.com/api/v1/metadata/network_chains")
            }
        }
    }

    pub fn get_status_text() -> String {
        match &*SETTINGS {
            Ok(settings) => format!(
                "Chainbase Network [{}] [{}] ",
                settings.app.network,
                settings.app.version
            ),
            Err(e) => {
                eprintln!("Failed to load settings: {}", e);
                String::from("Chainbase Network [Unknown] [Unknown]")
            }
        }
    }
}
