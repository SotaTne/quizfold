use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "quizfold")]
#[command(about = "QuizFold command line tools")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Validate { file: PathBuf },
    Preview { file: PathBuf },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Validate { file } => {
            let input = fs::read_to_string(file)?;
            let diagnostics = quizfold_parser::validate_quizfold(&input);
            if diagnostics.is_empty() {
                println!("ok");
            } else {
                for diagnostic in diagnostics {
                    println!(
                        "{} {:?}: {}",
                        diagnostic.code(),
                        diagnostic.severity(),
                        diagnostic.message()
                    );
                }
            }
        }
        Command::Preview { file } => {
            let input = fs::read_to_string(file)?;
            let result = quizfold_parser::parse_quizfold(&input);
            println!("{:#?}", result);
        }
    }

    Ok(())
}
