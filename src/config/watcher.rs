use anyhow::Result;
use notify::{Config, Error, Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;

pub fn run(tx: mpsc::Sender<Event>) -> Result<RecommendedWatcher> {
    let event_handler = move |res: Result<Event, notify::Error>| {
        let Ok(event) = res else {
            let err = res.unwrap_err();
            handler_config_not_found(&err);
            return;
        };

        if let Err(e) = tx.blocking_send(event) {
            tracing::warn!("file event send error: {}", e);
        }
    };

    let mut watcher = RecommendedWatcher::new(event_handler, Config::default())?;

    let path = super::AppConfig::path()?;

    if let Err(e) = watcher.watch(&path, RecursiveMode::NonRecursive) {
        handler_config_not_found(&e);
        if matches!(e.kind, notify::ErrorKind::PathNotFound) {
            watcher.watch(&path, RecursiveMode::NonRecursive)?;
        } else {
            return Err(e.into());
        }
    };

    Ok(watcher)
}

fn handler_config_not_found(err: &Error) {
    match err.kind {
        notify::ErrorKind::PathNotFound => {
            tracing::warn!("config file not found, generating default template...");
            if let Err(e) = super::AppConfig::create_default_template() {
                tracing::error!("failed to create default config: {}", e);
            } else {
                tracing::info!("default config generated");
            }
        }
        _ => {
            tracing::error!("file watch error: {:?}, kind: {:?}", err, err.kind);
        }
    }
}
