use std::{process::Command, path::PathBuf, io, collections::HashMap};
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
    Status(HashMap<String, JobStatus>),
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
        
        // Create manuscript directory
        let manuscript_dir = home_dir.join("manuscripts");
        std::fs::create_dir_all(&manuscript_dir)?;
        
        // Create demo directory inside manuscript
        let demo_dir = manuscript_dir.join("demo");
        std::fs::create_dir_all(&demo_dir)?;

        // Parse the YAML to get configuration values
        let (chain_name, table_name, database_name) = self.parse_manuscript_yaml(yaml_content)
            .unwrap_or(("solana".to_string(), "blocks".to_string(), "solana".to_string()));
        
        let os_type = if cfg!(target_os = "darwin") {
            "darwin"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else {
            "windows"
        };

        // Create config file content
        let config_content = format!(
            "baseDir    = {}\n\
            systemInfo = {}\n\n\
            [demo]\n\
            baseDir     = {}/manuscripts\n\
            name        = demo\n\
            specVersion = v1.0.0\n\
            parallelism = 1\n\
            chain       = {}\n\
            table       = {}\n\
            database    = {}\n\
            query       = Select * From {}_{}\n\
            sink        = postgres\n\
            port        = 8081\n\
            dbPort      = 15432\n\
            dbUser      = postgres\n\
            dbPassword  = postgres\n\
            graphqlPort = 9080",
            home_dir.display(),
            os_type,
            home_dir.display(),
            chain_name,
            table_name,
            database_name,
            database_name,
            table_name
        );

        // Write config file
        std::fs::write(home_dir.join(".manuscript_config.ini"), config_content)?;

        // Create docker-compose.yml
        self.create_docker_compose(&demo_dir, &database_name)?;

        // Start docker compose
        self.start_docker_compose(&demo_dir)?;

        Ok(())
    }

    fn create_docker_compose(&self, demo_dir: &std::path::Path, database_name: &str) -> io::Result<()> {
        let docker_compose_content = format!(
            "version: '3.4'\n\
name: demo
services:
  postgres:
    image: postgres:16.4
    ports:
      - \"15432:5432\"
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
      - \"9080:8080\"
    depends_on:
      - postgres
    environment:
      HASURA_GRAPHQL_DATABASE_URL: postgres://postgres:${{POSTGRES_PASSWORD:-postgres}}@postgres:5432/{}
      HASURA_GRAPHQL_ENABLE_CONSOLE: \"true\"
    networks:
      - ms_network
    restart: unless-stopped

networks:
  ms_network:",
            database_name,
            database_name,
            database_name
        );

        std::fs::write(demo_dir.join("docker-compose.yml"), docker_compose_content)?;
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

    fn parse_manuscript_yaml(&self, yaml_content: &str) -> Option<(String, String, String)> {
        if let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(yaml_content) {
            // Extract values from the YAML structure
            let dataset = yaml["sources"]
                .as_sequence()?
                .first()?
                ["dataset"]
                .as_str()?
                .to_string();
            
            // Split dataset into database and table names
            let parts: Vec<&str> = dataset.split('.').collect();
            if parts.len() == 2 {
                return Some((
                    parts[0].to_string(), // chain/database name
                    parts[1].to_string(), // table name
                    parts[0].to_string()  // database name
                ));
            }
        }
        None
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

    async fn check_jobs_status() -> Result<HashMap<String, JobStatus>, std::io::Error> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found"))?;
        
        let config_path = home_dir.join(".manuscript_config.ini");
        if !config_path.exists() {
            return Ok(HashMap::new());
        }
    
        let config_content = std::fs::read_to_string(config_path)?;
        let mut jobs = HashMap::new();
    
        // Parse INI file
        let mut current_section = "";
        let mut job_base_dir = "";
    
        for line in config_content.lines() {
            let line = line.trim();
            
            if line.starts_with("[") && line.ends_with("]") {
                current_section = &line[1..line.len()-1];
                if current_section != "demo" { continue; }
            } else if current_section == "demo" && line.starts_with("baseDir") {
                if let Some(dir) = line.split('=').nth(1) {
                    job_base_dir = dir.trim();
                }
            }
        }
    
        if current_section == "demo" && !job_base_dir.is_empty() {
            let job_dir = std::path::Path::new(job_base_dir).join("demo");
    
            // Change into the job's directory
            if let Err(e) = std::env::set_current_dir(&job_dir) {
                // println!("Failed to change directory to {}: {}", job_dir.display(), e);
                return Ok(jobs);
            }
    
            // Check docker compose status from within the directory
            let output = Command::new("docker")
                .args(["compose", "ps", "-a", "--format", "json"])
                .output()?;
    
    
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let mut containers = Vec::new();
                let mut all_running = true;
    
                for line in output_str.lines() {
                    if let Ok(container) = serde_json::from_str::<serde_json::Value>(line) {
                        let name = container["Name"].as_str().unwrap_or("").to_string();
                        let state = container["State"].as_str().unwrap_or("").to_string();
                        let status = container["RunningFor"].as_str().unwrap_or("").to_string();
                        
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
    
                let status = if all_running { JobState::Running } else { JobState::Pending };
    
                jobs.insert("demo".to_string(), JobStatus {
                    name: "demo".to_string(),
                    status,
                    containers,
                });
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