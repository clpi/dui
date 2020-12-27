pub mod config;
pub mod window;
pub mod models;
pub mod app;
pub mod ui;
pub mod state;
pub mod util;


pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    app::App::new().run()

}
