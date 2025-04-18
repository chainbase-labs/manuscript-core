use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::OnceLock;

use reqwest::blocking::Client;
use serde_json::Value;
use crate::logger::Logger;
use std::thread::sleep;
use std::time::Duration;

static API_PORT: OnceLock<u16> = OnceLock::new();

pub struct ApiServer {
    _child: Child,
    pub port: u16,
}

impl ApiServer {
    pub async fn start() -> Result<Self, String> {
        let mut child = Command::new("../common/api_server")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start API server");

        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);

        for line in reader.lines().flatten() {
            if let Some(port_str) = line.strip_prefix("API server started on port ") {
                let port = port_str.parse().expect("Invalid port number");

                let client = reqwest::Client::new();
                let url = format!("http://127.0.0.1:{}/health", port);

                for _ in 0..10 {
                    if let Ok(resp) = client.get(&url).send().await {
                        if resp.status().is_success() {
                            API_PORT.set(port).ok();
                            return Ok(ApiServer {
                                _child: child,
                                port,
                            });
                        }
                    }
                    tokio::time::sleep(Duration::from_millis(200)).await;
                }

                return Err("API server failed health check".to_string());
            }
        }

        Err("API server failed to start".to_string())
    }
}

impl Drop for ApiServer {
    fn drop(&mut self) {
        if let Err(e) = self._child.kill() {
            eprintln!("Failed to kill API server process: {}", e);
        } else {
            println!("API server process killed.");
        }
    }
}

pub async fn load_config(path: &str) -> Result<Value, String> {
    let port = API_PORT
        .get()
        .copied()
        .expect("API server not started yet");

    let url = format!("http://127.0.0.1:{}/load_config?path={}", port, path);
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    if resp.status().is_success() {
        resp.json().await.map_err(|e| format!("Invalid JSON: {}", e))
    } else {
        Err(format!("API returned error: {}", resp.status()))
    }
}


pub async fn list_job_statuses(path: &str) -> Result<Value, String> {
    let port = API_PORT
        .get()
        .copied()
        .expect("API server not started yet");

    let url = format!("http://127.0.0.1:{}/list_job_statuses?path={}", port, path);
    let client = reqwest::Client::new();

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;
    let status = resp.status();

    let body = resp
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    // eprintln!("[DEBUG] API Response Status: {:?}", status);
    // eprintln!("[DEBUG] API Response Body: {:?}", body);

    if status.is_success() {
        serde_json::from_str(&body).map_err(|e| format!("Invalid JSON: {}", e))
    } else {
        Err(format!("API returned error: {}", status))
    }
}