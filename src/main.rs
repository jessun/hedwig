use anyhow::{Ok, Result};
use notify::Event;
use tokio::sync::mpsc;

mod config;
mod logging;
mod poller;

fn main() -> Result<()> {
    logging::init();
    let git_hash = option_env!("GIT_HASH").unwrap_or("DEV");

    tracing::info!("hedwig({}) is running", git_hash);

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    rt.block_on(run())?;

    Ok(())
}

async fn run() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Event>(100);

    let _watcher = config::watcher::run(tx)?;
    poller::event_loop(rx).await?;
    Ok(())
}
