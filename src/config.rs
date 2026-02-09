use std::{fs, path::PathBuf};

use anyhow::{Context, Ok, Result, anyhow};
use serde::{Deserialize, Serialize};

pub mod watcher;
const DEFAULT_CONFIG_TEMPLATE: &str = r#"# ==========================================
# Hedwig Configuration File
# ==========================================

# [Required] Gmail Username
# Please enter your full email address.
username = "YOUR_GMAIL@gmail.com"

# [Required] Gmail App Password
# Note: This is NOT your standard Google login password.
# You must generate an App Password in your Google Account settings:
# Security -> 2-Step Verification -> App passwords
password = "YOUR_APP_PASSWORD"

# [Optional] HTTP Proxy Address
# Useful if you cannot connect to Gmail directly.
# Examples: "http://127.0.0.1:1087
# If you don't need a proxy, keep the line below commented out.
# proxy_addr = "http://127.0.0.1:7890"
"#;

pub mod defaults {
    #![allow(dead_code)]
    pub const USERNAME: &str = "YOUR_GMAIL@gmail.com";
    pub const PASSWORD: &str = "YOUR_APP_PASSWORD";
    pub const CFG_FILE_NAME: &str = "config.toml";
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppConfig {
    pub username: String,
    pub password: String,
    pub proxy_addr: Option<String>,
}

impl AppConfig {
    pub fn new() -> Self {
        AppConfig {
            username: defaults::USERNAME.into(),
            password: defaults::PASSWORD.into(),
            proxy_addr: None,
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
        let path = Self::path()?;
        if path.exists() {
            return Ok(());
        }

        fs::write(&path, DEFAULT_CONFIG_TEMPLATE).context("failed to write config file")?;

        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        !(self.username == defaults::USERNAME
            || self.password == defaults::PASSWORD
            || self.username.is_empty()
            || self.password.is_empty())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default_template() {
        let cfg: AppConfig = toml::from_str(DEFAULT_CONFIG_TEMPLATE)
            .expect("DEFAULT_CONFIG_TEMPLATE contains invalid TOML or mismatched fields");

        assert_eq!(cfg.username, defaults::USERNAME);
        assert_eq!(cfg.password, defaults::PASSWORD);
        assert_eq!(cfg.proxy_addr, None);
    }
}
