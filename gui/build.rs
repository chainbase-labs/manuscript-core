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

    let gui_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let common_dir = gui_root.join("../common");
    let output_path = gui_root.join("./target/release/api_server"); // ./gui/target/release/api_server

    let status = Command::new("go")
        .args(["build", "-o"])
        .arg(&output_path)
        .current_dir(&common_dir)
        .status()
        .expect("Failed to run go build");

    if !status.success() {
        panic!("Go build failed");
    }

    // === 通知 cargo 重新构建的触发条件 ===
    println!("cargo:rerun-if-changed=config/default.yaml");
    println!("cargo:rerun-if-changed=../common/*.go");
    println!("cargo:rustc-env=API_SERVER_PATH={}", output_path.display());
}