use std::{fs::File, sync::Mutex};

#[derive(Debug, Clone, clap::ValueEnum)]
pub(crate) enum LogLevel {
    Trace,
    Debug,
    Warn,
    Error,
}

pub(super) fn setup_logging(_log_level: Option<LogLevel>) {
    let log_file = File::create("/tmp/trace.log").expect("should create trace file");
    tracing_subscriber::fmt()
        // .with_env_filter(format!("smdrs={log_level:?}"))
        .with_env_filter(format!("smdrs=trace"))
        .with_writer(Mutex::new(log_file))
        .init();
}
