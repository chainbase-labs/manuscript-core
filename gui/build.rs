use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let config_dir = Path::new(&out_dir).parent().unwrap()
        .join("config");
    fs::create_dir_all(&config_dir).unwrap();

    fs::copy(
        "config/default.yaml",
        config_dir.join("default.yaml"),
    ).unwrap();
} 