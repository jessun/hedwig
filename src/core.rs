use anyhow::Result;
use notify::{
    Event,
    event::{DataChange, ModifyKind},
};
use tokio::{sync::mpsc, time};

use crate::{
    config::AppConfig,
    gmail::{client::GmailClient, parse::get_unread_count},
};

#[warn(unused_assignments)]
pub async fn event_loop(mut rx: mpsc::Receiver<Event>) -> Result<()> {
    tracing::info!("[poller] event loop initialized");

    let mut interval = time::interval(time::Duration::from_secs(60));
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    let mut cfg = AppConfig::load().unwrap_or_else(|e| {
        tracing::error!("failed to load config file: {}", e);
        AppConfig::new()
    });
    tracing::info!("load app config successfully. email_addr: {}", cfg.username);
    let client = GmailClient::new();

    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                if is_updated_file(event.kind) {
                    let res = AppConfig::load();
                    match res {
                        Err(e) => tracing::error!("failed to load config file: {}", e),
                        Ok(c) => {
                            if c.is_valid() {
                                cfg = c;
                                tracing::info!("update app config: email_addr: {}", cfg.username);
                                handler_gmail(&client, &cfg).await;
                                interval.reset();
                            }
                        }
                    }
                }
            }
            _ = interval.tick() => {
                tracing::debug!("tick!");
                if cfg.is_valid() {
                    handler_gmail(&client, &cfg).await;
                }
            }
        }
    }
}

fn is_updated_file(kind: notify::EventKind) -> bool {
    if matches!(
        kind,
        notify::EventKind::Modify(ModifyKind::Data(DataChange::Content))
    ) {
        return true;
    }
    false
}

async fn handler_gmail(client: &GmailClient, cfg: &AppConfig) {
    if let Err(e) = gmail_unread(client, cfg).await {
        tracing::error!("{}", e);
    }
}

async fn gmail_unread(client: &GmailClient, cfg: &AppConfig) -> Result<()> {
    let xml_resp = client.feed_atom(&cfg.username, &cfg.password).await?;
    let count = get_unread_count(&xml_resp)?;
    tracing::info!("{} unread mail count: {}", &cfg.username, count);
    Ok(())
}
