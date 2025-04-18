use std::collections::HashMap;
use manuscript_gui::app::App;
use manuscript_gui::logger::Logger;
use manuscript_gui::api::ApiServer;
use manuscript_gui::config::Settings;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if let Err(e) = Logger::init() {
        eprintln!("Failed to initialize logger: {}", e);
    }
    let _api_server = match ApiServer::start().await {
        Ok(server) => server,
        Err(e) => {
            eprintln!("Failed to start API server: {}", e);
            std::process::exit(1);
        }
    };

    let mut terminal = ratatui::init();
    let mut app = App::new().await;
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}