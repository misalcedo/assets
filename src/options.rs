use std::net::SocketAddr;
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
    /// Start the Import and GraphQL API server.
    Start(StartOptions),
}

#[derive(Debug, Parser)]
pub struct ImportOptions {
    /// The full URI of the server to POST to for import.
    #[arg(short, long, default_value = "http://127.0.0.1:2738/import")]
    pub uri: reqwest::Url,

    /// The file path to read the JSON file from. Defaults to STD_IN when a file is not given.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub struct StartOptions {
    /// The full URI of the server to POST to for import.
    #[arg(short, long, default_value = "127.0.0.1:2738")]
    pub address: SocketAddr,

    /// The file path for the DuckDB embedded database file.
    #[arg(short, long, default_value = "assets.db")]
    pub database_path: PathBuf,
}
