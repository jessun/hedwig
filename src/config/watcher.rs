use anyhow::Result;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;

pub fn run(tx: mpsc::Sender<Event>) -> Result<RecommendedWatcher> {
    let event_handler = move |res: Result<Event, notify::Error>| match res {
        Ok(event) => {
            if let Err(e) = tx.blocking_send(event) {
                tracing::error!("send error: {}", e);
            }
        }
        Err(e) => tracing::error!("watch error: {}", e),
    };

    let mut watcher = RecommendedWatcher::new(event_handler, Config::default())?;

    let path = super::Config::path()?;

    if let Err(e) = watcher.watch(&path, RecursiveMode::NonRecursive) {
        tracing::error!("file watch error: {}", e);
    };

    Ok(watcher)
}
