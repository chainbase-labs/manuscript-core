use manuscript_gui::app::App;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let mut terminal = ratatui::init();
    let mut app = App::new().await;
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}