use anyhow::Result;

use crate::{config::AppConfig, gmail::client::GmailClient};

pub struct ClientPool {
    inner: Option<GmailClient>,
    last_proxy_fingerprint: Option<String>,
}

impl ClientPool {
    pub fn new() -> Self {
        Self {
            inner: None,
            last_proxy_fingerprint: None,
        }
    }

    pub fn get(&mut self, cfg: &AppConfig) -> Result<GmailClient> {
        if !self.should_recreate(cfg) {
            return Ok(self.inner.as_ref().unwrap().clone());
        }

        tracing::info!("Proxy config changed or client invalid, refreshing client pool...");
        let new_client = GmailClient::new(cfg.proxy_addr.clone())?;
        self.inner = Some(new_client.clone());
        self.last_proxy_fingerprint = cfg.proxy_addr.clone();
        Ok(new_client)
    }

    fn should_recreate(&self, cfg: &AppConfig) -> bool {
        if self.inner.is_none() {
            return true;
        }
        if self.last_proxy_fingerprint != cfg.proxy_addr {
            return true;
        }
        false
    }
}
