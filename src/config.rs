use std::fs;
use std::path::{PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Output {
    pub dir: String
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    pub output: Output
}

pub fn parse_config(config_file: &PathBuf) -> Config {
    let config_text = fs::read_to_string(config_file).expect("Error reading file");
    toml::from_str(&config_text).expect("error reading stream")
}