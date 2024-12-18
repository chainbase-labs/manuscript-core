use std::{process::Command, io};
use webbrowser;
use tokio::sync::mpsc;
use tokio::time::Duration;
use super::docker::{DOCKER_COMPOSE_TEMPLATE, DOCKER_COMPOSE_TEMPLATE_SOLANA, JOB_CONFIG_TEMPLATE, MANUSCRIPT_TEMPLATE, MANUSCRIPT_TEMPLATE_SOLANA};
use crate::config::Settings;
use std::collections::HashSet;
#[derive(Debug, Clone)]
pub struct JobManager;

#[derive(Debug, Clone)]
pub struct JobStatus {
    pub name: String,
    pub status: JobState,
    pub containers: Vec<ContainerStatus>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContainerStatus {
    pub name: String,
    pub state: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JobState {
    Running,
    Pending,
    Failed,
    NotStarted,
    PullingImage,
    Creating,
}

#[derive(Debug)]
pub enum JobsCommand {
    Stop,
}

#[derive(Debug)]
pub enum JobsUpdate {
    Status(Vec<JobStatus>),
}

#[derive(Debug, Clone)]
pub struct ManuscriptConfig {
    pub name: String,
    pub spec_version: String,
    pub parallelism: u64,
    pub source: SourceConfig,
    pub transform: TransformConfig,
    pub sink: SinkConfig,
    pub db_port: u16,
    pub graphql_port: u16,
    pub job_port: u16,
}

#[derive(Debug, Clone)]
pub struct SourceConfig {
    pub name: String,
    pub dataset_type: String,
    pub dataset: String,
    // Parsed from dataset
    pub chain: String,
    pub table: String,
}

#[derive(Debug, Clone)]
pub struct TransformConfig {
    pub name: String,
    pub sql: String,
}

#[derive(Debug, Clone)]
pub struct SinkConfig {
    pub name: String,
    pub sink_type: String,
    pub from: String,
    pub database: String,
    pub schema: String,
    pub table: String,
    pub primary_key: String,
    pub config: DatabaseConfig,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl JobManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle_action(&self, job_name: &str, action: &str) -> io::Result<Option<String>> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
        let job_dir = home_dir.join("manuscripts").join(job_name);
        std::env::set_current_dir(&job_dir)?;

        match action {
            "edit" => {
                let yaml_content = std::fs::read_to_string(job_dir.join("manuscript.yaml"))?;
                Ok(Some(yaml_content))
            },
            "logs" => {
                let output = Command::new("docker")
                    .args(["compose", "logs"])
                    .output()?;
                
                if output.status.success() {
                    Ok(Some(String::from_utf8_lossy(&output.stdout).to_string()))
                } else {
                    Ok(Some(format!("Error: {}", String::from_utf8_lossy(&output.stderr))))
                }
            },
            "graphql" => {
                self.handle_graphql_action(job_name, &home_dir).await?;
                Ok(None)
            },
            "delete" => {
                // First, stop containers
                Command::new("docker")
                    .args(["compose", "down"])
                    .output()?;

                // Delete job directory
                std::fs::remove_dir_all(&job_dir)?;

                // Remove job configuration from .manuscript_config.ini
                let config_path = home_dir.join(".manuscript_config.ini");
                if config_path.exists() {
                    let content = std::fs::read_to_string(&config_path)?;
                    let mut lines: Vec<String> = Vec::new();
                    let mut skip_section = false;

                    for line in content.lines() {
                        if line.starts_with('[') && line.ends_with(']') {
                            skip_section = &line[1..line.len()-1] == job_name;
                        }
                        if !skip_section {
                            lines.push(line.to_string());
                        }
                    }

                    std::fs::write(config_path, lines.join("\n"))?;
                }

                Ok(None)
            },
            "start" => {
                Command::new("docker")
                    .args(["compose", "up", "-d"])
                    .output()?;
                Ok(None)
            },
            "stop" => {
                Command::new("docker")
                    .args(["compose", "down"])
                    .output()?;
                Ok(None)
            }
            _ => Ok(None)
        }
    }

    async fn handle_graphql_action(&self, job_name: &str, home_dir: &std::path::Path) -> io::Result<()> {
        let content = std::fs::read_to_string(home_dir.join(".manuscript_config.ini"))?;
        let mut current_section = "";
        let mut port = None;
        let mut table = None;
        let mut chain = None;

        for line in content.lines() {
            let line = line.trim();
            
            if line.starts_with("[") && line.ends_with("]") {
                current_section = &line[1..line.len()-1];
                continue;
            }
            
            if current_section == job_name {
                if line.starts_with("graphqlPort") {
                    if let Some(val) = line.split('=').nth(1) {
                        port = val.trim().parse::<u16>().ok();
                    }
                }
                if line.starts_with("table") {
                    if let Some(val) = line.split('=').nth(1) {
                        table = Some(val.trim().to_string());
                    }
                }
                if line.starts_with("chain") {
                    if let Some(val) = line.split('=').nth(1) {
                        chain = Some(val.trim().to_string());
                    }
                }
            }
        }

        if let (Some(port), Some(table), Some(chain)) = (port, table, chain) {
            let url = format!("http://127.0.0.1:{}", port);
            
            // TODO: The data here needs to be upgraded to automatically obtain from the protocol.
            let payload = if chain == "solana" {
                serde_json::json!({
                    "type": "bulk",
                    "source": "default", 
                    "resource_version": 1,
                    "args": [{
                        "type": "postgres_track_tables",
                        "args": {
                            "allow_warnings": true,
                            "tables": [
                                {"table": {"name": "blocks", "schema": "public"}, "source": "default"},
                                {"table": {"name": "cursors", "schema": "public"}, "source": "default"},
                                {"table": {"name": "mpl_token_metadata_create_metadata_account_v3_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "mpl_token_metadata_other_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "pumpfun_create_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "pumpfun_initialize_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "pumpfun_set_params_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "pumpfun_swap_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "pumpfun_withdraw_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "raydium_amm_deposit_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "raydium_amm_initialize_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "raydium_amm_swap_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "raydium_amm_withdraw_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "raydium_amm_withdraw_pnl_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "spl_token_approve_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "spl_token_burn_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "spl_token_close_account_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "spl_token_freeze_account_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "spl_token_initialize_account_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "spl_token_initialize_immutable_owner_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "spl_token_initialize_mint_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "spl_token_initialize_multisig_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "spl_token_mint_to_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "spl_token_revoke_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "spl_token_set_authority_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "spl_token_sync_native_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "spl_token_thaw_account_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "spl_token_transfer_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "substreams_history", "schema": "public"}, "source": "default"},
                                {"table": {"name": "system_program_advance_nonce_account_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "system_program_allocate_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "system_program_allocate_with_seed_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "system_program_assign_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "system_program_assign_with_seed_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "system_program_authorize_nonce_account_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "system_program_create_account_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "system_program_create_account_with_seed_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "system_program_initialize_nonce_account_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "system_program_transfer_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "system_program_transfer_with_seed_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "system_program_upgrade_nonce_account_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "system_program_withdraw_nonce_account_events", "schema": "public"}, "source": "default"},
                                {"table": {"name": "transactions", "schema": "public"}, "source": "default"}
                            ]
                        }
                    }]
                })
            } else {
                serde_json::json!({
                    "type": "bulk",
                    "source": "default",
                    "resource_version": 1,
                    "args": [{
                        "type": "postgres_track_tables",
                        "args": {
                            "allow_warnings": true,
                            "tables": [{
                                "table": {
                                    "name": table,
                                    "schema": "public"
                                },
                                "source": "default"
                            }]
                        }
                    }]
                })
            };

            let client = reqwest::Client::new();
            let response = client.post(format!("{}/v1/metadata", url))
                .json(&payload)
                .send()
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

            // Open browser after metadata request succeeds
            webbrowser::open(&url)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        }

        Ok(())
    }

    pub async fn create_config_file(&self, yaml_content: &str, tx: mpsc::Sender<JobsUpdate>) -> io::Result<()> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
        
        let config = self.parse_manuscript_yaml(yaml_content)?;
        let config_path = home_dir.join(".manuscript_config.ini");
        
        // Read existing config if it exists
        let existing_content = if config_path.exists() {
            std::fs::read_to_string(&config_path)?
        } else {
            // Initialize with system info if file doesn't exist
            let os_type = if cfg!(target_os = "macos") {
                "darwin"
            } else if cfg!(target_os = "linux") {
                "linux"
            } else {
                "darwin"
            };
            format!(
                "baseDir    = {}\nsystemInfo = {}\n",
                home_dir.display(),
                os_type
            )
        };

        // Create manuscript directory
        let manuscript_dir = home_dir.join("manuscripts");
        std::fs::create_dir_all(&manuscript_dir)?;
        
        // Create job directory inside manuscript
        let job_dir = manuscript_dir.join(&config.name);
        std::fs::create_dir_all(&job_dir)?;

        // Create manuscript.yaml file in the job directory
        std::fs::write(job_dir.join("manuscript.yaml"), yaml_content)?;

        // Parse existing content to find job sections
        let mut lines: Vec<String> = existing_content.lines().map(String::from).collect();
        let mut job_section_start = None;
        let mut job_section_end = None;
        let mut current_section = String::new();
        
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with('[') && line.ends_with(']') {
                if !current_section.is_empty() && job_section_start.is_some() {
                    job_section_end = Some(i);
                }
                current_section = line[1..line.len()-1].to_string();
                if current_section == config.name {
                    job_section_start = Some(i);
                }
            }
        }


        let job_port = self.get_available_port(18080, 18090)
        .unwrap_or(18080);

        
        // Create new job config content
        let new_job_config = JOB_CONFIG_TEMPLATE
            .replace("{name}", &config.name)
            .replace("{home_dir}", &home_dir.display().to_string())
            .replace("{spec_version}", &config.spec_version)
            .replace("{parallelism}", &config.parallelism.to_string())
            .replace("{chain}", &config.source.chain)
            .replace("{job_port}", &job_port.to_string())
            .replace("{graphql_port}", &config.graphql_port.to_string())
            .replace("{table}", &config.source.table)
            .replace("{database}", &config.sink.database)
            .replace("{query}", &config.transform.sql)
            .replace("{sink_type}", &config.sink.sink_type)
            .replace("{db_port}", &config.db_port.to_string())
            .replace("{db_user}", &config.sink.config.username)
            .replace("{db_password}", &config.sink.config.password);

        // Update or append the job configuration
        if let (Some(start), Some(end)) = (job_section_start, job_section_end) {
            // Replace existing job section
            lines.splice(start..end, new_job_config.lines().map(String::from));
        } else if let Some(start) = job_section_start {
            // Replace until the end of file
            lines.truncate(start);
            lines.extend(new_job_config.lines().map(String::from));
        } else {
            // Append new job section
            lines.extend(new_job_config.lines().map(String::from));
        }

        // Write updated content back to file
        let updated_content = lines.join("\n");
        std::fs::write(&config_path, updated_content)?;

        // Create and start docker-compose with the correct name
        self.create_docker_compose(&job_dir, &config)?;

        let _ = tx.send(JobsUpdate::Status(vec![JobStatus {
            name: config.name,
            status: JobState::Creating,
            containers: Vec::new(),
        }])).await;

        self.start_docker_compose(&job_dir)?;
        // thread::sleep(Duration::from_secs(10));

        Ok(())
    }

    fn create_docker_compose(&self, job_dir: &std::path::Path, config: &ManuscriptConfig) -> io::Result<()> {
        let (mut job_manager_image, hasura_image) = Settings::get_docker_images();

        // TODO: solana support needs change the job_manager image
        // Solana compatible with future needs to migrate to the refactored protocol.
        if config.source.chain == "solana" {
            job_manager_image = "repository.chainbase.com/manuscript-node/manuscript-solana:latest".to_string();
        }

        let template = if config.source.chain == "solana" {
            DOCKER_COMPOSE_TEMPLATE_SOLANA
        } else {
            DOCKER_COMPOSE_TEMPLATE
        };
        
        let docker_compose_content = template
            .replace("{name}", &config.name)
            .replace("{job_manager_image}", &job_manager_image)
            .replace("{hasura_image}", &hasura_image)
            .replace("{database}", &config.sink.database)
            .replace("{db_port}", &config.db_port.to_string())
            .replace("{graphql_port}", &config.graphql_port.to_string())
            .replace("{job_port}", &config.job_port.to_string());

        std::fs::write(job_dir.join("docker-compose.yml"), docker_compose_content)?;
        Ok(())
    }

    fn start_docker_compose(&self, demo_dir: &std::path::Path) -> io::Result<()> {
        // Change to the demo directory
        std::env::set_current_dir(demo_dir)?;

        // Run docker compose up -d
        let output = Command::new("docker")
            .args(["compose", "up", "-d"])
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Failed to start docker compose: {}",
                    String::from_utf8_lossy(&output.stderr)
                )
            ));
        }

        Ok(())
    }

    fn parse_manuscript_yaml(&self, yaml_content: &str) -> Result<ManuscriptConfig, io::Error> {
        let yaml: serde_yaml::Value = serde_yaml::from_str(yaml_content)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to parse YAML: {}", e)))?;

        // Parse source configuration
        let source = yaml["sources"]
            .as_sequence()
            .and_then(|sources| sources.first())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No source configuration found"))?;

        let dataset = source["dataset"]
            .as_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Dataset not found"))?;
        
        let parts: Vec<&str> = dataset.split('.').collect();
        if parts.len() != 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid dataset format"));
        }

        // Parse transform configuration
        let transform = yaml["transforms"]
            .as_sequence()
            .and_then(|transforms| transforms.first())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No transform configuration found"))?;

        // Parse sink configuration
        let sink = yaml["sinks"]
            .as_sequence()
            .and_then(|sinks| sinks.first())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No sink configuration found"))?;

        let config = ManuscriptConfig {
            name: yaml["name"].as_str().unwrap_or("demo").to_string(),
            spec_version: yaml["specVersion"].as_str().unwrap_or("v1.0.0").to_string(),
            parallelism: yaml["parallelism"].as_u64().unwrap_or(1),
            db_port: self.get_available_port(15432, 15439)
                .unwrap_or(15432),
            graphql_port: self.get_available_port(19080, 19090)
                .unwrap_or(19080),
            job_port: self.get_available_port(18080, 18090)
                .unwrap_or(18080),
            source: SourceConfig {
                name: source["name"].as_str().unwrap_or("").to_string(),
                dataset_type: source["type"].as_str().unwrap_or("dataset").to_string(),
                dataset: dataset.to_string(),
                chain: parts[0].to_string(),
                table: parts[1].to_string(),
            },
            transform: TransformConfig {
                name: transform["name"].as_str().unwrap_or("").to_string(),
                sql: transform["sql"].as_str().unwrap_or("").to_string(),
            },
            sink: SinkConfig {
                name: sink["name"].as_str().unwrap_or("").to_string(),
                sink_type: sink["type"].as_str().unwrap_or("postgres").to_string(),
                from: sink["from"].as_str().unwrap_or("").to_string(),
                database: sink["database"].as_str().unwrap_or("").to_string(),
                schema: sink["schema"].as_str().unwrap_or("public").to_string(),
                table: sink["table"].as_str().unwrap_or("").to_string(),
                primary_key: sink["primary_key"].as_str().unwrap_or("").to_string(),
                config: DatabaseConfig {
                    host: sink["config"]["host"].as_str().unwrap_or("postgres").to_string(),
                    port: sink["config"]["port"].as_u64().unwrap_or(5432) as u16,
                    username: sink["config"]["username"].as_str().unwrap_or("postgres").to_string(),
                    password: sink["config"]["password"].as_str().unwrap_or("postgres").to_string(),
                },
            },
        };

        Ok(config)
    }

    pub fn transform_yaml_to_sql(&self, yaml_content: &str) -> Result<String, String> {
        let yaml: serde_yaml::Value = serde_yaml::from_str(yaml_content)
            .map_err(|e| format!("Failed to parse YAML: {}", e))?;

        let dataset = yaml["sources"]
            .as_sequence()
            .and_then(|sources| sources.first())
            .and_then(|source| source["dataset"].as_str())
            .ok_or_else(|| "Failed to extract dataset".to_string())?;

        let sql = yaml["transforms"]
            .as_sequence()
            .and_then(|transforms| transforms.first())
            .and_then(|transform| transform["sql"].as_str())
            .ok_or_else(|| "Failed to extract SQL query".to_string())?;

        let sql = sql.trim();

        // Check if SQL already contains the full dataset name (including schema)
        let sql = if sql.contains(dataset) {
            format!("select * from ({}) limit 10", sql)
        } else {
            // Use the full dataset name which includes schema
            format!("select * from {} limit 10", dataset)
        };

        Ok(sql)
    }

    async fn check_jobs_status() -> Result<Vec<JobStatus>, std::io::Error> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found"))?;
        
        let config_path = home_dir.join(".manuscript_config.ini");
        if !config_path.exists() {
            return Ok(Vec::new());
        }

        let config_content = std::fs::read_to_string(config_path)?;
        let mut jobs = Vec::new();
        let mut current_section = String::new();
        let mut current_job_base_dir = String::new();
        let mut current_job_name = String::new();

        // Parse INI file to get all jobs in order
        for line in config_content.lines() {
            let line = line.trim();
            
            if line.starts_with('[') && line.ends_with(']') {
                // New section
                current_section = line[1..line.len()-1].to_string();
                continue;
            }

            if !current_section.is_empty() {
                if line.starts_with("baseDir") {
                    if let Some(dir) = line.split('=').nth(1) {
                        current_job_base_dir = dir.trim().to_string();
                    }
                } else if line.starts_with("name") {
                    if let Some(name) = line.split('=').nth(1) {
                        current_job_name = name.trim().to_string();
                    }
                }

                // If we have both baseDir and name, we can check the job status
                if !current_job_base_dir.is_empty() && !current_job_name.is_empty() {
                    let job_dir = std::path::Path::new(&current_job_base_dir)
                        .join(&current_job_name);

                    // Change into the job's directory
                    if let Ok(_) = std::env::set_current_dir(&job_dir) {
                        // Check docker compose status
                        if let Ok(output) = Command::new("docker")
                            .args(["compose", "ps", "-a", "--format", "json"])
                            .output()
                        {
                            if output.status.success() {
                                let output_str = String::from_utf8_lossy(&output.stdout);
                                let mut containers = Vec::new();
                                let mut all_running = true;
                                let mut has_containers = false;

                                for line in output_str.lines() {
                                    if let Ok(container) = serde_json::from_str::<serde_json::Value>(line) {
                                        has_containers = true;
                                        let name = container["Name"].as_str().unwrap_or("").to_string();
                                        let state = container["State"].as_str().unwrap_or("").to_string();
                                        let status = container["RunningFor"].as_str().unwrap_or("").to_string();

                                        let status = if !status.is_empty() {
                                            format!("({})", status)
                                        } else {
                                            status
                                        };
                                        if state != "running" {
                                            all_running = false;
                                        }
                                        
                                        containers.push(ContainerStatus { 
                                            name, 
                                            state,
                                            status 
                                        });
                                    }
                                }

                                let status = if !has_containers {
                                    JobState::NotStarted
                                } else if all_running {
                                    JobState::Running 
                                } else { 
                                    JobState::Pending 
                                };

                                jobs.push(JobStatus {
                                    name: current_job_name.clone(),
                                    status,
                                    containers,
                                });
                            }
                        }
                    }

                    // Reset for next job
                    current_job_base_dir.clear();
                    current_job_name.clear();
                }
            }
        }

        Ok(jobs)
    }

    pub async fn jobs_monitor(
        &self,
        mut command_rx: mpsc::Receiver<JobsCommand>,
        status_tx: mpsc::Sender<JobsUpdate>
    ) {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
    
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if let Ok(status) = Self::check_jobs_status().await {
                        let _ = status_tx.send(JobsUpdate::Status(status)).await;
                    }
                }
                Some(JobsCommand::Stop) = command_rx.recv() => {
                    break;
                }
            }
        }
    }

    fn get_available_port(&self, start: u16, end: u16) -> io::Result<u16> {
        // First check ports in manuscript_config.ini
        let home_dir = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
        let config_path = home_dir.join(".manuscript_config.ini");
        
        let mut used_ports = HashSet::new();
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            for line in content.lines() {
                if line.contains("dbPort") || line.contains("graphqlPort") {
                    if let Some(port_str) = line.split('=').nth(1) {
                        if let Ok(port) = port_str.trim().parse::<u16>() {
                            used_ports.insert(port);
                        }
                    }
                }
            }
        }

        // Check system ports using lsof command
        if let Ok(output) = Command::new("lsof")
            .args(["-nP", "-iTCP", "-sTCP:LISTEN"])
            .output() 
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines().skip(1) { // Skip header line
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 9 {
                    if let Some(port_str) = parts[8].split(':').last() {
                        if let Ok(port) = port_str.trim_end_matches("(LISTEN)").parse::<u16>() {
                            used_ports.insert(port);
                        }
                    }
                }
            }
        }

        // Find first available port in range
        for port in start..=end {
            if !used_ports.contains(&port) {
                return Ok(port);
            }
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("No available ports in range {}-{}", start, end)
        ))
    }

    pub fn generate_initial_manuscript(&self, dataset_name: &str, table_name: &str) -> String {
        let table_name = if table_name == "transactionLogs" {
            "transaction_logs"
        } else {
            table_name
        };

        // Get available ports
        let db_port = self.get_available_port(15432, 15439)
            .unwrap_or(15432);
        let graphql_port = self.get_available_port(19080, 19090)
            .unwrap_or(19080);

        // TODO: solana support while moving to the refactored protocol
        let manuscript = if dataset_name == "solana" {
            MANUSCRIPT_TEMPLATE_SOLANA
        } else {
            MANUSCRIPT_TEMPLATE
        };

        manuscript
            .replace("{name}", "demo")
            .replace("{dataset_name}", dataset_name)
            .replace("{table_name}", table_name)
            .replace("{db_port}", &db_port.to_string())
            .replace("{graphql_port}", &graphql_port.to_string())
    }
}
