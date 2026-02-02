use std::{fs, path::PathBuf};

use anyhow::{Context, Ok, Result, anyhow};
use serde::{Deserialize, Serialize};

pub mod watcher;

pub mod defaults {
    #![allow(dead_code)]
    pub const USERNAME: &str = "YOUR_GMAIL@gmail.com";
    pub const PASSWORD: &str = "YOUR_APP_PASSWORD";
    pub const CFG_FILE_NAME: &str = "config.toml";
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub proxy_addr: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: defaults::USERNAME.into(),
            password: defaults::PASSWORD.into(),
            proxy_addr: String::new(),
        }
    }
}

impl Config {
    fn path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir().ok_or_else(|| anyhow!("could not find home directory"))?;
        let config_dir = home_dir.join(".config").join(env!("CARGO_PKG_NAME"));
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).with_context(|| "failed to create config directory")?;
        }

        Ok(config_dir.join(defaults::CFG_FILE_NAME))
    }
}
