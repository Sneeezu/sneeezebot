mod commands;
mod config;
mod events;
mod utils;

use config::Config;
use events::Handler;

#[tokio::main]
pub async fn main() {
    let config_path = dirs::config_dir()
        .expect("Failed to determine config directory")
        .join("sneeezebot")
        .join("config.toml");
    let cfg =
        Config::from_file(&config_path).expect("Couldn't create new config from a config file");
    let handler = Handler::new(cfg);

    handler.handle_events().await.unwrap();
}
