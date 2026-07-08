// Entry point for the repository maintenance binary.
// It parses xtask CLI arguments and routes them to command modules.
mod cli;
mod commands;
mod tools;

use anyhow::Result;
use clap::Parser;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = cli::Cli::parse();
    cli.run()
}
