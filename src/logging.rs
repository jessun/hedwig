use std::fs;

use anyhow::{Context, Error, Result, anyhow};
use flexi_logger::{
    Cleanup, Criterion, FileSpec, LogSpecification, Naming, WriteMode,
    trc::{FormatConfig, SpecFileNotifier},
    writers::{FileLogWriter, FileLogWriterHandle},
};

pub fn init() -> Result<(FileLogWriterHandle, SpecFileNotifier), Error> {
    let log_dir = dirs::home_dir()
        .ok_or_else(|| anyhow!("could find home dir"))
        .map(|home| home.join(".local/state/").join(env!("CARGO_PKG_NAME")))?;

    if !log_dir.exists() {
        fs::create_dir_all(&log_dir).context("failed to create state dir")?;
        tracing::info!("create dir {:?} successfully", log_dir);
    }

    let log_sepc = LogSpecification::env_or_parse("debug")?;
    let flw = FileLogWriter::builder(
        FileSpec::default()
            .directory(log_dir)
            .basename(env!("CARGO_PKG_NAME")),
    )
    .rotate(
        Criterion::Size(10 * 1024),
        Naming::Timestamps,
        Cleanup::KeepLogFiles(7),
    )
    .write_mode(WriteMode::Async);
    let fmt = FormatConfig::default().with_time(true);

    let (log_handler, spec_file_notifier) =
        flexi_logger::trc::setup_tracing(log_sepc, None, flw, &fmt)?;
    Ok((log_handler, spec_file_notifier))
}
