use anyhow::Result;
use notify::{
    Event,
    event::{DataChange, ModifyKind},
};
use tokio::{sync::mpsc, time};

use crate::config::AppConfig;

pub async fn event_loop(mut rx: mpsc::Receiver<Event>) -> Result<()> {
    tracing::info!("[poller] event loop initialized");

    let mut interval = time::interval(time::Duration::from_secs(60));
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    let mut cfg = AppConfig::new();

    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                if is_updated_file( event.kind) {
                    let res = AppConfig::load();
                    match res {
                        Err(e) => tracing::error!("failed to load config file: {}", e),
                        Ok(c) => {
                            if c.is_valid() {
                                cfg = c;
                                tracing::info!("update app config: email_addr: {}", cfg.username);
                            }
                        }
                    }
                }
            }
            _ = interval.tick() => {
            // mail feed atom HTTP request
            }
        }
    }
}

fn is_updated_file(kind: notify::EventKind) -> bool {
    if matches!(
        kind,
        notify::EventKind::Modify(ModifyKind::Data(DataChange::Content))
    ) {
        tracing::debug!("kind: {:?}", kind);
        return true;
    }
    false
}
