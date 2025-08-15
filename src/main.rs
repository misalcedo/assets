mod commands;
pub(crate) mod import;
mod options;
mod verbose;

use clap::Parser;
use options::Options;
use verbose::set_log_level;
use crate::options::Commands;

fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    set_log_level(options.verbosity)?;

    tracing::debug!(?options, "Invoking command");

    match options.command {
        Commands::Import(import_options) => commands::import_assets(&import_options),
    }
}
