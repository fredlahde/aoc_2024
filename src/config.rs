use std::{fs::File, io::Read};

use anyhow::{Context, Result};
use constcat::concat;
use serde_derive::Deserialize;

pub const STORAGE_DIR: &str = "storage/";
pub const PROMPTS_DIR: &str = concat!(STORAGE_DIR, "prompts");
pub const INPUTS_DIR: &str = concat!(STORAGE_DIR, "inputs");
pub const SAMPLES_DIR: &str = concat!(STORAGE_DIR, "samples");

#[derive(Deserialize)]
pub struct Config {
    pub session_cookie: String,
}

pub fn load_config() -> Result<Config> {
    let mut fd = File::open("config.toml").context("failed to open config file")?;
    let mut content = String::new();
    fd.read_to_string(&mut content)
        .context("failed to read config to string")?;

    toml::from_str(&content).context("failed to deserialize config")
}
