pub mod app;
pub mod cli;
pub mod config;
pub mod executor;
pub mod ui;
pub mod url_scheme;

use anyhow::Result;

pub use app::ClientLinkerApp;
pub use config::AppConfig;

pub fn run_gui(url: Option<String>) -> Result<()> {
    ui::run_gui(url)
}
