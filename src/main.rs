mod api;
mod db;
mod commands;
mod http;
mod import;
mod options;
mod verbose;

use clap::Parser;
use options::Options;
use verbose::set_log_level;
use crate::options::Commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    set_log_level(options.verbosity)?;

    tracing::debug!(?options, "Invoking command");

    match options.command {
        Commands::Import(import_options) => commands::import_assets(&import_options).await,
        Commands::Start(start_options) => http::start_server(&start_options).await,
    }
}
