mod application;
mod components;
mod core;
mod logging;

use crate::logging::LogLevel;
use clap::Parser;
use tracing::trace;

#[derive(Clone, Debug, clap::ValueEnum)]
enum ClientType {
    Dummy,
    Firewall,
}

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    log_level: Option<LogLevel>,

    #[arg(value_enum)]
    client_type: ClientType,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    logging::setup_logging(args.log_level);
    trace!("logging configured");

    // TODO Setup puffin

    application::start(args.client_type)
}
