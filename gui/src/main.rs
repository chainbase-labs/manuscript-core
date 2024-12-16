use manuscript_gui::app::App;
use manuscript_gui::logger::Logger;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if let Err(e) = Logger::init() {
        eprintln!("Failed to initialize logger: {}", e);
    }
    let mut terminal = ratatui::init();
    let mut app = App::new().await;
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}