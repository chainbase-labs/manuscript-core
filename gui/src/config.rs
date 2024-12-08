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
pub struct NodeConfig {
    pub job_manager_image_arm64: String,
    pub hasura_image_arm64: String,
    pub job_manager_image_amd64: String,
    pub hasura_image_amd64: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub api: ApiConfig,
    pub app: AppConfig,
    pub node: NodeConfig,
}

lazy_static! {
    pub static ref SETTINGS: Result<Settings, ConfigError> = Settings::new();
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut builder = Config::builder();
        
        let default_config = include_str!("../config/default.yaml");
        builder = builder
            .add_source(config::File::from_str(
                default_config,
                config::FileFormat::Yaml
            ));

        let exe_path = std::env::current_exe()
            .unwrap_or_else(|_| std::path::PathBuf::from("."));
        let exe_dir = exe_path.parent().unwrap_or_else(|| std::path::Path::new("."));

        let config_locations = vec![
            "config/default.yaml".to_string(),
            "../config/default.yaml".to_string(),
            "../../config/default.yaml".to_string(),
            format!("{}/config/default.yaml", exe_dir.display()),
        ];

        for location in config_locations {
            let path = Path::new(&location);
            if path.exists() {
                builder = builder.add_source(File::with_name(&location).format(FileFormat::Yaml));
                break;
            }
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
                "[? Help] Chainbase Network [{}] [{}] ",
                settings.app.network,
                settings.app.version
            ),
            Err(e) => {
                eprintln!("Failed to load settings: {}", e);
                String::from("[? Help] Chainbase Network [Unknown] [Unknown]")
            }
        }
    }

    pub fn get_docker_images() -> (String, String) {
        match &*SETTINGS {
            Ok(settings) => {
                let is_arm = cfg!(target_arch = "aarch64");
                if is_arm {
                    (
                        settings.node.job_manager_image_arm64.clone(),
                        settings.node.hasura_image_arm64.clone(),
                    )
                } else {
                    (
                        settings.node.job_manager_image_amd64.clone(),
                        settings.node.hasura_image_amd64.clone(),
                    )
                }
            }
            Err(e) => {
                eprintln!("Failed to load settings: {}", e);
                (
                    "repository.chainbase.com/manuscript-node/manuscript-node:latest".to_string(),
                    "repository.chainbase.com/manuscript-node/graphql-engine-amd64:latest".to_string(),
                )
            }
        }
    }

    pub fn get_chainbase_url() -> String {
        match &*SETTINGS {
            Ok(settings) => settings.api.base_url.clone(),
            Err(e) => {
                eprintln!("Failed to load settings: {}", e);
                String::from("https://api.chainbase.com")
            }
        }
    }
}
