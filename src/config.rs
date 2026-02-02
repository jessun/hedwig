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
pub struct AppConfig {
    pub username: String,
    pub password: String,
    pub proxy_addr: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            username: defaults::USERNAME.into(),
            password: defaults::PASSWORD.into(),
            proxy_addr: String::new(),
        }
    }
}

impl AppConfig {
    pub fn new() -> Self {
        AppConfig {
            username: String::new(),
            password: String::new(),
            proxy_addr: String::new(),
        }
    }
    fn path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir().ok_or_else(|| anyhow!("could not find home directory"))?;
        let config_dir = home_dir.join(".config").join(env!("CARGO_PKG_NAME"));
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).with_context(|| "failed to create config directory")?;
        }

        Ok(config_dir.join(defaults::CFG_FILE_NAME))
    }
    fn create_default_template() -> Result<()> {
        let cfg = Self::default();

        let toml_str = toml::to_string_pretty(&cfg)?;
        let path = Self::path()?;
        fs::write(path, toml_str)?;
        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        self.username == defaults::USERNAME
            || self.password == defaults::PASSWORD
            || self.username.is_empty()
            || self.password.is_empty()
    }

    pub fn load() -> Result<Self> {
        let file_path = Self::path()?;

        if !file_path.exists() {
            return Err(anyhow!("config file not exist"));
        }

        let content =
            fs::read_to_string(&file_path).with_context(|| "failed to read config file")?;
        let cfg: AppConfig =
            toml::from_str(&content).with_context(|| "failed to parse config file")?;

        Ok(cfg)
    }
}
