use serde::Deserialize;
use std::{error::Error, fs, path::Path};

#[derive(Deserialize)]
pub struct Config {
    pub prefix: String,
    pub twitch: Twitch,
    pub channels: Vec<Channel>,
}

#[derive(Deserialize)]
pub struct Twitch {
    pub login: String,
    pub oauth: Option<String>,
}

#[derive(Deserialize)]
pub struct Channel {
    // TODO: use uids instead
    pub login: String,
}

// TODO: handle when the file does not exist and stuff?
impl Config {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;

        match toml::from_str(&contents) {
            Ok(cfg) => Ok(cfg),
            Err(err) => Err(Box::new(err)),
        }
    }
}
