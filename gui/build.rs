use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::io;

fn main() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let base_dir = Path::new(&out_dir).parent().unwrap().to_path_buf();

    let config_dir = base_dir.join("config");
    fs::create_dir_all(&config_dir)?;
    fs::copy("config/default.yaml", config_dir.join("default.yaml"))?;
    println!("cargo:rerun-if-changed=config/default.yaml");

    let api_src_dir = Path::new("api");
    let api_dst_dir = base_dir.join("api");
    fs::create_dir_all(&api_dst_dir)?;

    copy_dir_recursive(api_src_dir, &api_dst_dir)?;
    println!("cargo:rerun-if-changed=api");

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if path.is_dir() {
            fs::create_dir_all(&dst_path)?;
            copy_dir_recursive(&path, &dst_path)?;
        } else {
            fs::copy(&path, &dst_path)?;
        }
    }
    Ok(())
}