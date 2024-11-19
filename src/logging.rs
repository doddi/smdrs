#[derive(Debug, Clone, clap::ValueEnum)]
pub(crate) enum LogLevel {
    Trace,
    Debug,
    Warn,
    Error,
}

pub(super) fn setup_logging(_log_level: Option<LogLevel>) {}
