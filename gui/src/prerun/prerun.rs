use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;
use tokio::sync::mpsc;
use crate::app::AppUpdate;
use crate::app::SetupStep;
use crate::app::SetupStepStatus;
use base64::{Engine as _, engine::general_purpose::STANDARD};

#[derive(Debug)]
pub struct PreRun {
    #[allow(dead_code)]
    image: String,
    api_endpoint: String,
    catalog_statement: String,
    un: String,
    pw: String,
}

impl PreRun {
    pub fn new() -> Self {
        Self {
            image: "repository.chainbase.com/manuscript-node/manuscript-debug:latest".to_string(),
            api_endpoint: "https://testnet-debug.chainbasehq.com".to_string(),
            catalog_statement: r#"CREATE CATALOG paimon WITH ( 
                'type' = 'paimon',
                'warehouse' = 'oss://network-testnet/warehouse',
                'table-default.merge-engine' = 'deduplicate',
                'table-default.changelog-producer' = 'input',
                'table-default.metastore.partitioned-table' = 'false',
                'table-default.lookup.cache-file-retention' = '1 h',
                'table-default.lookup.cache-max-memory-size' = '256 mb',
                'table-default.lookup.cache-max-disk-size' = '10 gb',
                'table-default.log.scan.remove-normalize' = 'true',
                'table-default.changelog-producer.row-deduplicate' = 'false',
                'table-default.consumer.expiration-time' = '24 h',
                'table-default.streaming-read-mode' = 'file',
                'table-default.orc.bloom.filter.fpp' = '0.00001',
                'table-default.scan.plan-sort-partition' = 'true',
                'table-default.snapshot.expire.limit' = '10000',
                'table-default.snapshot.num.retained.max' = '2000'
            );"#.to_string(),
            un: "un".to_string(),
            pw: "pww".to_string(),
        }
    }

    pub fn set_auth(&mut self, un: String, pw: String) {
        self.un = un;
        self.pw = pw;
    }

    fn get_auth_header(&self) -> String {
        let credentials = format!("{}:{}", self.un, self.pw);
        format!("Basic {}", STANDARD.encode(credentials))
    }

    pub async fn setup(&self, sender: Option<mpsc::Sender<AppUpdate>>, sql: Option<String>) -> Result<String, String> {
        // Step 1: Submit SQL task
        if let Some(sender) = &sender {
            let _ = sender.send(AppUpdate::SetupProgress(SetupStep::SubmitSQLTask, SetupStepStatus::InProgress)).await;
        }
        let (session_handle, operation_handle) = self.submit_sql_task(sql.as_deref()).await?;

        // Step 2: Wait for execution results
        if let Some(sender) = &sender {
            let _ = sender.send(AppUpdate::SetupProgress(SetupStep::WaitingForExecutionResults, SetupStepStatus::InProgress)).await;
        }
        let results = self.wait_for_results(&session_handle, &operation_handle).await?;

        Ok(format!("Setup completed successfully. Results:\n{}", results))
    }

    async fn submit_sql_task(&self, sql: Option<&str>) -> Result<(String, String), String> {
        // Step 1: Get session
        let session_handle = self.create_session().await?;
        
        // Step 2: Create catalog
        self.create_catalog(&session_handle).await?;
        
        // Step 3: Use catalog
        self.use_catalog(&session_handle).await?;

        // Step 4: Set runtime mode to batch
        self.set_runtime_mode(&session_handle).await?;
        
        // Step 5: Submit SQL query
        let operation_handle = self.submit_query(&session_handle, sql).await?;
        
        Ok((session_handle, operation_handle))
    }

    async fn create_session(&self) -> Result<String, String> {
        let client = reqwest::Client::new();
        let mut attempts = 0;
        let max_attempts = 10;

        loop {
            match client.post(format!("{}/v1/sessions", self.api_endpoint))
                .header("Authorization", self.get_auth_header())
                .send()
                .await {
                    Ok(response) => {
                        let text = response.text().await.unwrap_or_else(|e| format!("Failed to get response text: {}", e));
                        match serde_json::from_str::<serde_json::Value>(&text) {
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

            sleep(Duration::from_secs(1)).await;
        }
    }

    async fn create_catalog(&self, session_handle: &str) -> Result<(), String> {
        let client = reqwest::Client::new();
        
        let response = client.post(format!("{}/v1/sessions/{}/statements", self.api_endpoint, session_handle))
            .header("Content-Type", "application/json")
            .header("Authorization", self.get_auth_header())
            .json(&serde_json::json!({ "statement": self.catalog_statement }))
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

        sleep(Duration::from_secs(1)).await;
        Ok(())
    }

    async fn use_catalog(&self, session_handle: &str) -> Result<(), String> {
        let client = reqwest::Client::new();
        let response = client.post(format!("{}/v1/sessions/{}/statements", self.api_endpoint, session_handle))
            .header("Content-Type", "application/json")
            .header("Authorization", self.get_auth_header())
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

        // sleep(Duration::from_secs(1)).await;
        Ok(())
    }

    async fn submit_query(&self, session_handle: &str, sql: Option<&str>) -> Result<String, String> {
        let client = reqwest::Client::new();
        let url = format!("{}/v1/sessions/{}/statements", self.api_endpoint, session_handle);

        // Check if SQL is empty
        let statement = match sql {
            Some(s) if !s.trim().is_empty() => s,
            _ => return Err("No SQL query provided".to_string())
        };

        let body = serde_json::json!({ "statement": statement });

        let mut retries = 0;
        let max_retries = 30;
        
        loop {
            match client.post(&url)
                .header("Content-Type", "application/json")
                .header("Authorization", self.get_auth_header())
                .json(&body)
                .send()
                .await {
                    Ok(response) => {
                        match response.json::<serde_json::Value>().await {
                            Ok(json) => {
                                if let Some(errors) = json.get("errors") {
                                    return Err(format!("Query submission failed: {}", errors));
                                }

                                return json.get("operationHandle")
                                    .and_then(|h| h.as_str())
                                    .map(String::from)
                                    .ok_or_else(|| "Invalid operation handle format".to_string())
                            },
                            Err(e) => {
                                if retries >= max_retries {
                                    return Err(format!("Failed to parse query response after {} retries: {}", max_retries, e));
                                }
                            }
                        }
                    },
                    Err(e) => {
                        if retries >= max_retries {
                            return Err(format!("Failed to submit query after {} retries: {}", max_retries, e));
                        }
                    }
            }

            retries += 1;
            sleep(Duration::from_secs(1)).await;
        }
    }

    async fn wait_for_results(&self, session_handle: &str, operation_handle: &str) -> Result<serde_json::Value, String> {
        let client = reqwest::Client::new();
        let mut next_uri = format!("/v1/sessions/{}/operations/{}/result/0", session_handle, operation_handle);
        let mut retries = 0;
        let max_retries = 60;

        loop {
            if retries >= max_retries {
                return Err(format!("Timeout waiting for results after {} retries", max_retries));
            }

            let response = client.get(format!("{}{}", self.api_endpoint, next_uri))
                .header("Authorization", self.get_auth_header())
                .send()
                .await
                .map_err(|e| format!("Failed to fetch results: {}", e))?;

            let json: serde_json::Value = response.json()
                .await
                .map_err(|e| format!("Failed to parse results: {}", e))?;

            // Check for errors
            if let Some(errors) = json.get("errors") {
                let error_str = errors.to_string();
                // Use raw string to handle escape sequences
                let lines: Vec<&str> = error_str.split(r"\n").collect();
                let mut caused_by_errors = Vec::new();
                
                for line in lines {
                    if line.starts_with("Caused by:") {
                        caused_by_errors.push(line.to_string());
                    }
                }

                if !caused_by_errors.is_empty() {
                    return Err(caused_by_errors.join(r"\n")); 
                } else {
                    return Err(format!("Error in results: {}", errors));
                }
            }

            match json.get("resultType").and_then(|rt| rt.as_str()) {
                Some("NOT_READY") => {
                    if let Some(next) = json.get("nextResultUri").and_then(|uri| uri.as_str()) {
                        next_uri = next.to_string();
                    }
                },
                Some("EOS") => {
                    return Ok(serde_json::json!("No results found"));
                },
                Some("PAYLOAD") => {
                    if let Some(data) = json.get("results") {
                        // Check if data field is not empty
                        if data.get("data").and_then(|d| d.as_array()).map_or(false, |arr| !arr.is_empty()) {
                            // Extract column names and types
                            let columns = data.get("columns")
                                .and_then(|cols| cols.as_array())
                                .map(|cols| cols.iter()
                                    .filter_map(|col| {
                                        let name = col.get("name").and_then(|n| n.as_str())?;
                                        let type_ = col.get("logicalType")
                                            .and_then(|t| t.get("type"))
                                            .and_then(|t| t.as_str())?;
                                        Some((name, type_))
                                    })
                                    .collect::<Vec<_>>()
                                );

                            // Extract data rows
                            let rows = data.get("data")
                                .and_then(|d| d.as_array())
                                .map(|rows| rows.iter()
                                    .filter_map(|row| row.get("fields").and_then(|f| f.as_array()))
                                    .collect::<Vec<_>>()
                                );

                            if let (Some(cols), Some(rows)) = (columns, rows) {
                                let mut result = String::new();
                                
                                // Format each row
                                for (row_num, row) in rows.iter().enumerate() {
                                    result.push_str(&format!("Row {}:\n", row_num + 1));
                                    result.push_str("------\n");
                                    
                                    for ((col_name, col_type), value) in cols.iter().zip(row.iter()) {
                                        result.push_str(&format!("{:<20} | {:<15} | {}\n",
                                            col_name,
                                            col_type,
                                            value.to_string().trim_matches('"')
                                        ));
                                    }
                                    result.push('\n');
                                }

                                return Ok(serde_json::json!(result));
                            }
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
            retries += 1;
            sleep(Duration::from_secs(2)).await;
        }
    }

    async fn set_runtime_mode(&self, session_handle: &str) -> Result<(), String> {
        let client = reqwest::Client::new();
        let response = client.post(format!("{}/v1/sessions/{}/statements", self.api_endpoint, session_handle))
            .header("Content-Type", "application/json")
            .header("Authorization", self.get_auth_header())
            .json(&serde_json::json!({ "statement": "SET 'execution.runtime-mode' = 'batch';" }))
            .send()
            .await
            .map_err(|e| format!("Failed to set runtime mode: {}", e))?;

        // Check for errors in response
        let json: serde_json::Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse runtime mode response: {}", e))?;

        if let Some(errors) = json.get("errors") {
            return Err(format!("Set runtime mode failed: {}", errors));
        }

        // sleep(Duration::from_secs(1)).await;
        Ok(())
    }
}

impl Clone for PreRun {
    fn clone(&self) -> Self {
        Self {
            image: self.image.clone(),
            api_endpoint: self.api_endpoint.clone(),
            catalog_statement: self.catalog_statement.clone(),
            un: self.un.clone(),
            pw: self.pw.clone(),
        }
    }
} 