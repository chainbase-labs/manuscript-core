use std::fs::File;
use std::path::PathBuf;
use simplelog::*;
use chrono::Local;
use std::io;
use time::macros::format_description;

pub struct Logger;

impl Logger {
    pub fn init() -> io::Result<()> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
        
        let log_dir = home_dir.join(".manuscript").join("logs");
        std::fs::create_dir_all(&log_dir)?;
        
        let log_file_name = format!("manuscript_{}.log", Local::now().format("%Y-%m-%d"));
        let log_file_path = log_dir.join(log_file_name);

        println!("log_file_path: {}", log_file_path.to_string_lossy());
        
        let config = ConfigBuilder::new()
            .set_time_format_custom(format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"))
            .set_thread_level(LevelFilter::Info)
            .set_target_level(LevelFilter::Error)
            .set_location_level(LevelFilter::Debug)
            .build();

        CombinedLogger::init(vec![
            WriteLogger::new(
                LevelFilter::Info,
                config.clone(),
                File::create(log_file_path)?
            ),
        ]).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(())
    }

    pub fn get_log_path() -> io::Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
        
        let log_dir = home_dir.join(".manuscript").join("logs");
        let log_file_name = format!("manuscript_{}.log", Local::now().format("%Y-%m-%d"));
        
        Ok(log_dir.join(log_file_name))
    }
}
