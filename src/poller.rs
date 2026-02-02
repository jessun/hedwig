use anyhow::Result;
use notify::Event;
use tokio::{sync::mpsc, time};

pub async fn event_loop(mut rx: mpsc::Receiver<Event>) -> Result<()> {
    tracing::info!("[poller] event loop initialized");

    let mut interval = time::interval(time::Duration::from_secs(60));
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    loop {
        tokio::select! {
                Some(event) = rx.recv() => {
                    if  is_updated_file( event.kind) {
                        tracing::info!("config file changed: {:?}",event.kind);
                    }
                }

                _ = interval.tick() => {
                    // mail feed atom HTTP request
            }
        }
    }
}

fn is_updated_file(kind: notify::EventKind) -> bool {
    if matches!(kind, notify::EventKind::Modify(_)) {
        return true;
    }
    false
}
