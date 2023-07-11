mod commands;
mod config;
mod events;
mod utils;

use config::Config;
use events::Handler;
use std::process;

#[tokio::main]
pub async fn main() {
    let config_path = match dirs::config_dir() {
        Some(p) => p,
        None => {
            eprintln!("Failed to determine config directory");
            process::exit(1);
        }
    }
    .join("sneeezebot")
    .join("config.toml");

    let cfg = Config::from_file(&config_path).unwrap_or_else(|e| {
        eprintln!("Failed to create new config from a config file: {e}");
        process::exit(1);
    });

    let handler = Handler::new(cfg);
    if let Err(e) = handler.handle_events().await {
        eprintln!("{e}")
    }
}
