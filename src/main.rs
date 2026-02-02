mod config;
mod logging;

fn main() {
    logging::init();
    let git_hash = option_env!("GIT_HASH").unwrap_or("DEV");
    tracing::info!("[Version: {}] Hello world!", git_hash);
}
