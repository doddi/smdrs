mod application;
mod components;
mod core;
mod logging;

use crate::logging::LogLevel;
use clap::Parser;
use tracing::trace;

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    log_level: Option<LogLevel>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    logging::setup_logging(args.log_level);
    trace!("logging configured");

    // TODO Setup puffin

    application::start()
}
