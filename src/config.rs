use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub default_python_version: String,
    pub venv_path: String,
    pub mode: Mode,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Mode {
    System,
    Portable,
}

impl Config {
    /// Load configuration from a file or create a default one
    pub fn load(config_path: &Path) -> Self {
        if config_path.exists() {
            let data = utils::read_file(config_path).unwrap();
            serde_json::from_str(&data).unwrap()
        } else {
            Config {
                default_python_version: "3.9".to_string(),
                venv_path: if cfg!(windows) {
                    "C:\\PyRo\\venvs".to_string()
                } else {
                    "/usr/local/pyro/venvs".to_string()
                },
                mode: Mode::System,
            }
        }
    }

    /// Save configuration to a file
    pub fn save(&self, config_path: &Path) -> Result<(), std::io::Error> {
        let data = serde_json::to_string_pretty(self)?;
        utils::write_file(config_path, &data)
    }
}