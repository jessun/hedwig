use anyhow::Result;
use notify::{
    Event,
    event::{DataChange, ModifyKind},
};
use tokio::{sync::mpsc, time};

use crate::{config::AppConfig, gmail, notifier, pool::ClientPool};

#[warn(unused_assignments)]
pub async fn event_loop(mut rx: mpsc::Receiver<Event>) -> Result<()> {
    tracing::info!("[poller] event loop initialized");
    notifier::init();

    let mut interval = time::interval(time::Duration::from_secs(60));
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    let mut cfg = AppConfig::load().unwrap_or_else(|e| {
        tracing::error!("failed to load config file: {}", e);
        AppConfig::default()
    });
    tracing::info!("load app config successfully. email_addr: {}", cfg.username);
    let mut client_pool = ClientPool::new();

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
                                notifier::send("Configuration Updated", "New settings have been loaded successfully.");
                                tracing::info!("update app config: email_addr: {}", cfg.username);
                                handler_gmail(&mut client_pool, &cfg).await;
                                interval.reset();
                            }
                        }
                    }
                }
            }
            _ = interval.tick() => {
                tracing::debug!("tick!");
                if cfg.is_valid() {
                    handler_gmail(&mut client_pool, &cfg).await;
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

async fn handler_gmail(pool: &mut ClientPool, cfg: &AppConfig) {
    if let Err(e) = gmail_unread(pool, cfg).await {
        tracing::error!("{}", e);
    }
}

async fn gmail_unread(pool: &mut ClientPool, cfg: &AppConfig) -> Result<()> {
    let client = pool.get(cfg)?;
    let xml_resp = client.feed_atom(&cfg.username, &cfg.password).await?;
    let count = gmail::parse::get_unread_count(&xml_resp)?;
    notifier::send(
        "Unread Email Reminder",
        format!("You have {} unread emails.", count).as_ref(),
    );
    Ok(())
}
