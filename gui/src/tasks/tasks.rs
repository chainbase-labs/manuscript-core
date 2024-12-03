use std::{process::Command, path::PathBuf, io, collections::HashMap, collections::BTreeMap};
use webbrowser;
use tokio::sync::mpsc;
use tokio::time::Duration;

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
                // Read the manuscript.yaml file
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
            },
            "graphql" => {
                if let Some(port) = self.get_job_graphql_port(job_name)? {
                    let url = format!("http://127.0.0.1:{}", port);
                    webbrowser::open(&url)
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
                }
                Ok(None)
            }
            _ => Ok(None)
        }
    }

    fn get_job_graphql_port(&self, job_name: &str) -> io::Result<Option<u16>> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;

        let content = std::fs::read_to_string(home_dir.join(".manuscript_config.ini"))?;
        
        let mut current_section = "";
        for line in content.lines() {
            let line = line.trim();
            
            if line.starts_with("[") && line.ends_with("]") {
                current_section = &line[1..line.len()-1];
                continue;
            }
            
            if current_section == job_name && line.starts_with("graphqlPort") {
                if let Some(port_str) = line.split('=').nth(1) {
                    if let Ok(port) = port_str.trim().parse::<u16>() {
                        return Ok(Some(port));
                    }
                }
            }
        }
        Ok(None)
    }

    pub fn create_config_file(&self, yaml_content: &str) -> io::Result<()> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
        
        let config = self.parse_manuscript_yaml(yaml_content)?;
        let config_path = home_dir.join(".manuscript_config.ini");
        
        // Read existing config if it exists
        let mut existing_content = if config_path.exists() {
            std::fs::read_to_string(&config_path)?
        } else {
            // Initialize with system info if file doesn't exist
            let os_type = if cfg!(target_os = "darwin") {
                "darwin"
            } else if cfg!(target_os = "linux") {
                "linux"
            } else {
                "windows"
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
        
        // Create new job config content
        let new_job_config = format!(
            "\n[{}]\n\
            baseDir     = {}/manuscripts\n\
            name        = {}\n\
            specVersion = {}\n\
            parallelism = {}\n\
            chain       = {}\n\
            table       = {}\n\
            database    = {}\n\
            query       = {}\n\
            sink        = {}\n\
            port        = 8081\n\
            dbPort      = {}\n\
            dbUser      = {}\n\
            dbPassword  = {}\n\
            graphqlPort = 9080",
            config.name,
            home_dir.display(),
            config.name,
            config.spec_version,
            config.parallelism,
            config.source.chain,
            config.source.table,
            config.sink.database,
            config.transform.sql,
            config.sink.sink_type,
            config.sink.config.port,
            config.sink.config.username,
            config.sink.config.password,
        );

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
        self.create_docker_compose(&job_dir, &config.name, &config.sink.database)?;
        self.start_docker_compose(&job_dir)?;

        Ok(())
    }

    fn create_docker_compose(&self, job_dir: &std::path::Path, name: &str, database_name: &str) -> io::Result<()> {
        let docker_compose_content = format!(
            "version: '3.4'\n\
name: {}\nservices:",
            name
        ) + &format!(
            r#"
  postgres:
    image: postgres:16.4
    ports:
      - "15432:5432"
    volumes:
      - ./postgres_data:/var/lib/postgresql/data
    environment:
      - POSTGRES_PASSWORD=${{POSTGRES_PASSWORD:-postgres}}
      - POSTGRES_USER=${{POSTGRES_USER:-postgres}}
      - POSTGRES_DB=${{POSTGRES_DB:-{}}}
    networks:
      - ms_network
    restart: unless-stopped

  jobmanager:
    image: repository.chainbase.com/manuscript-node/manuscript-{}:latest
    networks:
      - ms_network

  hasura:
    image: repository.chainbase.com/manuscript-node/graphql-engine-arm64:latest
    ports:
      - "9080:8080"
    depends_on:
      - postgres
    environment:
      HASURA_GRAPHQL_DATABASE_URL: postgres://postgres:${{POSTGRES_PASSWORD:-postgres}}@postgres:5432/{}
      HASURA_GRAPHQL_ENABLE_CONSOLE: "true"
    networks:
      - ms_network
    restart: unless-stopped

networks:
  ms_network:"#,
            database_name,
            database_name,
            database_name
        );

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
                                    JobState::Failed
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
}
