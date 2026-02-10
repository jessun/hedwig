use std::str::FromStr;

use anyhow::Result;
use chrono::Local;
use cron::Schedule;
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

    let mut cfg = AppConfig::load().unwrap_or_else(|e| {
        tracing::error!("failed to load config file: {}", e);
        AppConfig::default()
    });
    tracing::info!("load app config successfully. email_addr: {}", cfg.username);
    let mut client_pool = ClientPool::new();
    handler_gmail(&mut client_pool, &cfg).await;

    loop {
        let mut delay = calculate_next_delay(&cfg.cron_expr);

        tokio::select! {
            Some(event) = rx.recv() => {
                if is_updated_file(event.kind) {
                    let res = AppConfig::load();
                    match res {
                        Err(e) => tracing::error!("failed to load config file: {}", e),
                        Ok(c) => {
                            if c.is_valid() {
                                cfg = c;
                                delay = calculate_next_delay(&cfg.cron_expr);
                                notifier::send("Configuration Updated", "New settings have been loaded successfully.");
                                tracing::info!("update app config: email_addr: {}", cfg.username);
                                handler_gmail(&mut client_pool, &cfg).await;
                            }
                        }
                    }
                }
            }
           _ = time::sleep(delay) => {
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

fn calculate_next_delay(expr: &str) -> time::Duration {
    let schedule = match Schedule::from_str(expr) {
        Ok(s) => s,
        Err(e) => {
            tracing::error!(
                "Invalid cron expression '{}': {}. Fallback to 60s.",
                expr,
                e
            );
            return time::Duration::from_secs(600);
        }
    };
    if let Some(next_event) = schedule.upcoming(Local).next() {
        let now = Local::now();
        if let Ok(duration) = (next_event - now).to_std() {
            tracing::info!("Next check scheduled at: {}", next_event);
            return duration;
        }
    }
    time::Duration::from_secs(600)
}
