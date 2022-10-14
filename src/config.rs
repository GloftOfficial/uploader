use std::path::PathBuf;

use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub bind: String,
    pub file_dir: PathBuf,
    pub domain: String,
    pub authorization: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    toml::from_slice(
        &std::fs::read("config.toml").expect("A config.toml must be in the working directory"),
    )
    .expect("The config.toml must be valid")
});
