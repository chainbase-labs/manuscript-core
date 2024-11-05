use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;
use tokio::sync::mpsc;
use crate::app::AppUpdate;
use crate::app::SetupStep;
use crate::app::SetupStepStatus;

#[derive(Debug)]
pub struct DockerManager {
    image: String,
    api_endpoint: String,
}

impl DockerManager {
    pub fn new() -> Self {
        Self {
            image: "repository.chainbase.com/manuscript-node/manuscript-debug:latest".to_string(),
            api_endpoint: "http://127.0.0.1:18083".to_string(),
        }
    }

    pub async fn setup(&self, sender: Option<mpsc::Sender<AppUpdate>>) -> Result<String, String> {
        // Step 1: Check Docker installation
        if let Some(sender) = &sender {
            let _ = sender.send(AppUpdate::SetupProgress(SetupStep::CheckingDocker, SetupStepStatus::InProgress)).await;
        }
        if !self.check_docker_installed() {
            return Err("Docker is not installed or not accessible".to_string());
        }

        // Step 2: Pull the image
        if let Some(sender) = &sender {
            let _ = sender.send(AppUpdate::SetupProgress(SetupStep::PullingImage, SetupStepStatus::InProgress)).await;
        }
        if let Err(e) = self.pull_image().await {
            return Err(format!("Failed to pull image: {}", e));
        }

        // Step 3: Run the container
        if let Some(sender) = &sender {
            let _ = sender.send(AppUpdate::SetupProgress(SetupStep::StartingContainer, SetupStepStatus::InProgress)).await;
        }
        if let Err(e) = self.run_container().await {
            return Err(format!("Failed to start container: {}", e));
        }

        // Step 4: Submit SQL task
        if let Some(sender) = &sender {
            let _ = sender.send(AppUpdate::SetupProgress(SetupStep::SubmitSQLTask, SetupStepStatus::InProgress)).await;
        }
        let (session_handle, operation_handle) = self.submit_sql_task().await?;

        // Step 5: Wait for execution results
        if let Some(sender) = &sender {
            let _ = sender.send(AppUpdate::SetupProgress(SetupStep::WaitingForExecutionResults, SetupStepStatus::InProgress)).await;
        }
        let results = self.wait_for_results(&session_handle, &operation_handle).await?;

        Ok(format!("Setup completed successfully. Results: {}", results))
    }

    fn check_docker_installed(&self) -> bool {
        Command::new("docker")
            .arg("--version")
            .output()
            .is_ok()
    }

    async fn pull_image(&self) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["pull", &self.image])
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    async fn run_container(&self) -> Result<(), String> {
        // Check if container already exists
        let check_output = Command::new("docker")
            .args(["ps", "-q", "-f", "name=manuscript-debug"])
            .output()
            .map_err(|e| e.to_string())?;

        // If container exists (output not empty), return success
        if !String::from_utf8_lossy(&check_output.stdout).trim().is_empty() {
            return Ok(());
        }

        // Container doesn't exist, create and run it
        let output = Command::new("docker")
            .args([
                "run",
                "-d",  // Run in detached mode
                "--rm",
                "--name",
                "manuscript-debug", 
                "-p", "18083:8083",
                "-p", "18081:8081",
                &self.image,
            ])
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        // Wait for container to be ready
        sleep(Duration::from_secs(5)).await;
        Ok(())
    }

    async fn submit_sql_task(&self) -> Result<(String, String), String> {
        // Step 1: Get session
        let session_handle = self.create_session().await?;
        
        // Step 2: Create catalog
        self.create_catalog(&session_handle).await?;
        
        // Step 3: Use catalog
        self.use_catalog(&session_handle).await?;
        
        // Step 4: Submit SQL query
        let operation_handle = self.submit_query(&session_handle).await?;
        
        Ok((session_handle, operation_handle))
    }

    async fn create_session(&self) -> Result<String, String> {
        let client = reqwest::Client::new();
        let mut attempts = 0;
        let max_attempts = 5;

        loop {
            match client.post(format!("{}/v1/sessions", self.api_endpoint))
                .send()
                .await {
                    Ok(response) => {
                        match response.json::<serde_json::Value>().await {
                            Ok(json) => {
                                if let Some(handle) = json.get("sessionHandle").and_then(|h| h.as_str()) {
                                    return Ok(handle.to_string());
                                }
                                attempts += 1;
                            },
                            Err(e) => {
                                attempts += 1;
                                if attempts == max_attempts {
                                    return Err(format!("Failed to parse session response after {} attempts: {}", max_attempts, e));
                                }
                            }
                        }
                    },
                    Err(e) => {
                        attempts += 1;
                        if attempts == max_attempts {
                            return Err(format!("Failed to create session after {} attempts: {}", max_attempts, e));
                        }
                    }
            }

            if attempts >= max_attempts {
                return Err(format!("Failed to create valid session after {} attempts", max_attempts));
            }

            sleep(Duration::from_secs(5)).await;
        }
    }

    async fn create_catalog(&self, session_handle: &str) -> Result<(), String> {
        let client = reqwest::Client::new();
        // TODO: add catalog
        let statement = r#""#;

        let response = client.post(format!("{}/v1/sessions/{}/statements", self.api_endpoint, session_handle))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({ "statement": statement }))
            .send()
            .await
            .map_err(|e| format!("Failed to create catalog: {}", e))?;

        // Check for errors in response
        let json: serde_json::Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse catalog response: {}", e))?;

        if let Some(errors) = json.get("errors") {
            return Err(format!("Catalog creation failed: {}", errors));
        }

        sleep(Duration::from_secs(5)).await;
        Ok(())
    }

    async fn use_catalog(&self, session_handle: &str) -> Result<(), String> {
        let client = reqwest::Client::new();
        let response = client.post(format!("{}/v1/sessions/{}/statements", self.api_endpoint, session_handle))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({ "statement": "use catalog paimon;" }))
            .send()
            .await
            .map_err(|e| format!("Failed to use catalog: {}", e))?;

        // Check for errors in response
        let json: serde_json::Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse use catalog response: {}", e))?;

        if let Some(errors) = json.get("errors") {
            return Err(format!("Use catalog failed: {}", errors));
        }

        sleep(Duration::from_secs(5)).await;
        Ok(())
    }

    async fn submit_query(&self, session_handle: &str) -> Result<String, String> {
        let client = reqwest::Client::new();
        let response = client.post(format!("{}/v1/sessions/{}/statements", self.api_endpoint, session_handle))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({ "statement": "select * from zkevm.blocks limit 10" }))
            .send()
            .await
            .map_err(|e| format!("Failed to submit query: {}", e))?;

        let json: serde_json::Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse query response: {}", e))?;

        if let Some(errors) = json.get("errors") {
            return Err(format!("Query submission failed: {}", errors));
        }

        json.get("operationHandle")
            .and_then(|h| h.as_str())
            .map(String::from)
            .ok_or_else(|| "Invalid operation handle format".to_string())
    }

    async fn wait_for_results(&self, session_handle: &str, operation_handle: &str) -> Result<serde_json::Value, String> {
        let client = reqwest::Client::new();
        let mut next_uri = format!("/v1/sessions/{}/operations/{}/result/0", session_handle, operation_handle);

        loop {
            let response = client.get(format!("{}{}", self.api_endpoint, next_uri))
                .send()
                .await
                .map_err(|e| format!("Failed to fetch results: {}", e))?;

            let json: serde_json::Value = response.json()
                .await
                .map_err(|e| format!("Failed to parse results: {}", e))?;

            // Check for errors
            if let Some(errors) = json.get("errors") {
                return Err(format!("Error in results: {}", errors));
            }

            match json.get("resultType").and_then(|rt| rt.as_str()) {
                Some("NOT_READY") => {
                    if let Some(next) = json.get("nextResultUri").and_then(|uri| uri.as_str()) {
                        next_uri = next.to_string();
                    }
                },
                Some("PAYLOAD") => {
                    if let Some(data) = json.get("results").and_then(|r| r.get("data")) {
                        if !data.as_array().map_or(true, |arr| arr.is_empty()) {
                            return Ok(json);
                        }
                    }
                    if let Some(next) = json.get("nextResultUri").and_then(|uri| uri.as_str()) {
                        next_uri = next.to_string();
                    }
                },
                rt => {
                    return Err("Unknown result type".to_string())
                }
            }
            sleep(Duration::from_secs(5)).await;
        }
    }
}

impl Clone for DockerManager {
    fn clone(&self) -> Self {
        Self::new()
    }
} 