use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let config_dir = Path::new(&out_dir).parent().unwrap().join("config");

    fs::create_dir_all(&config_dir).expect("Failed to create config dir");

    fs::copy(
        "config/default.yaml",
        config_dir.join("default.yaml"),
    ).expect("Failed to copy default.yaml");

    let api_dir=Path::new(&out_dir).parent().unwrap().join("api");
    fs::create_dir_all(&api_dir).expect("Failed to create api dir");

    fs::copy(
        "api/api_server",
        api_dir.join("api_server"),
    ).expect("Failed to copy api_server");

    println!("cargo:rerun-if-changed=config/default.yaml");
    println!("cargo:rerun-if-changed=api/api_server");
}