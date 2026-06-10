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
