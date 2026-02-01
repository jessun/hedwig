use tracing_subscriber::{EnvFilter, FmtSubscriber, fmt::time::OffsetTime};

pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("error"));
    let timer = OffsetTime::local_rfc_3339().expect("could not get local offset!");

    let subscriber = FmtSubscriber::builder()
        .with_timer(timer)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
