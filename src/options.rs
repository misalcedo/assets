use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about)]
pub struct Options {
    /// Sets the verbosity of logging.
    #[arg(short = 'v', long = None, action = clap::ArgAction::Count, global = true)]
    pub verbosity: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Import assets from a JSON file.
    Import(ImportOptions),
}

#[derive(Debug, Parser)]
pub struct ImportOptions {
    /// The file path to read the JSON file from. Defaults to STD_IN when a file is not given.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}
