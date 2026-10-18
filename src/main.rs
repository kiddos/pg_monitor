use anyhow::Result;
use clap::Parser;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

mod config;
mod db;
mod model;
mod tui;

#[cfg(test)]
mod tests;

use config::Config;
use db::DbClient;
use tui::{run_app, App};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::parse();

    let pg_config = match config.build_pg_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Invalid PostgreSQL configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Try connecting to database
    let db_client = match DbClient::connect(&pg_config).await {
        Ok(client) => Some(client),
        Err(e) => {
            eprintln!("Warning: Initial database connection failed: {:#}", e);
            None
        }
    };

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let app = App::new(config, pg_config);
    let res = run_app(&mut terminal, app, db_client).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Application error: {:?}", err);
    }

    Ok(())
}
