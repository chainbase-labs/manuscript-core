use std::io;
use ratatui::DefaultTerminal;

mod app;
mod ui;
mod setup;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let mut terminal = ratatui::init();
    let mut app = app::App::new().await;
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
