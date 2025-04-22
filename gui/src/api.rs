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
const EMBEDDED_API: &[u8] = include_bytes!("../api/api_server");

impl ApiServer {
    pub async fn start() -> Result<Self, String> {
        let exe_path = std::env::current_exe()
            .unwrap_or_else(|_| std::path::PathBuf::from("."));
        let exe_dir = exe_path.parent().unwrap_or_else(|| std::path::Path::new("."));

        let candidates = vec![
            "api/api_server".to_string(),
            "../api/api_server".to_string(),
            "../../api/api_server".to_string(),
            format!("{}/api/api_server", exe_dir.display()),
        ];

        let mut api_path: Option<std::path::PathBuf> = None;
        for candidate in candidates {
            eprintln!("{:?}",candidate);
            let candidate_path = std::path::Path::new(&candidate).to_path_buf();
            if candidate_path.exists() {
                api_path = Some(candidate_path);
                break;
            }
        }

        let final_path = match api_path {
            Some(p) => p,
            None => {
                println!("The size of the embedded API is: {} bytes", EMBEDDED_API.len());
                let temp_path = std::env::temp_dir().join("embedded_api_server");
                eprintln!("Temp file will be written to: {}", temp_path.display());
                std::fs::write(&temp_path, EMBEDDED_API).expect("Failed to write embedded API binary");

                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = std::fs::metadata(&temp_path).unwrap().permissions();
                    perms.set_mode(0o755);
                    std::fs::set_permissions(&temp_path, perms).unwrap();
                }

                temp_path
            }
        };

        let mut child = Command::new(final_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start API server");

        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);

        for line in reader.lines().flatten() {
            if let Some(port_str) = line.strip_prefix("API server started on port ") {
                let port = port_str.parse().expect("Invalid port number");

                let client = reqwest::Client::builder()
                    .no_proxy()
                    .build()
                    .map_err(|e| format!("Failed to build client: {}", e))?;
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
        let stderr = child.stderr.take().unwrap();
        let stderr_reader = BufReader::new(stderr);
        tokio::spawn(async move {
            for line in stderr_reader.lines().flatten() {
                eprintln!("[api_server stderr] {}", line);
            }
        });


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
    let client = reqwest::Client::builder()
        .no_proxy()
        .build()
        .map_err(|e| format!("Failed to build client: {}", e))?;
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
    let client = reqwest::Client::builder()
        .no_proxy()
        .build()
        .map_err(|e| format!("Failed to build client: {}", e))?;

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