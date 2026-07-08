// CLI shape for repository automation tasks.
// This file defines command names, arguments, and target enums for xtask.
use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

use crate::commands;

#[derive(Debug, Parser)]
#[command(name = "xtask")]
#[command(about = "QuizFold repository automation")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Build {
        #[command(subcommand)]
        command: BuildCommand,
    },
    Check {
        #[command(subcommand)]
        command: CheckCommand,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum BuildCommand {
    Wasm,
    Cli {
        #[arg(
            value_enum,
            value_name = "TARGET",
            required = true,
            num_args = 1..
        )]
        selections: Vec<CliBuildSelection>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub(crate) enum CliBuildSelection {
    All,
    #[value(name = "aarch64-apple-darwin")]
    Aarch64AppleDarwin,
    #[value(name = "x86_64-apple-darwin")]
    X86_64AppleDarwin,
    #[value(name = "aarch64-unknown-linux-gnu")]
    Aarch64UnknownLinuxGnu,
    #[value(name = "x86_64-unknown-linux-gnu")]
    X86_64UnknownLinuxGnu,
    #[value(name = "aarch64-pc-windows-msvc")]
    Aarch64PcWindowsMsvc,
    #[value(name = "x86_64-pc-windows-msvc")]
    X86_64PcWindowsMsvc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CliBuildTarget {
    Aarch64AppleDarwin,
    X86_64AppleDarwin,
    Aarch64UnknownLinuxGnu,
    X86_64UnknownLinuxGnu,
    Aarch64PcWindowsMsvc,
    X86_64PcWindowsMsvc,
}

impl CliBuildSelection {
    pub fn expand(self) -> Vec<CliBuildTarget> {
        match self {
            Self::All => CliBuildTarget::all_targets().to_vec(),
            Self::Aarch64AppleDarwin => vec![CliBuildTarget::Aarch64AppleDarwin],
            Self::X86_64AppleDarwin => vec![CliBuildTarget::X86_64AppleDarwin],
            Self::Aarch64UnknownLinuxGnu => vec![CliBuildTarget::Aarch64UnknownLinuxGnu],
            Self::X86_64UnknownLinuxGnu => vec![CliBuildTarget::X86_64UnknownLinuxGnu],
            Self::Aarch64PcWindowsMsvc => vec![CliBuildTarget::Aarch64PcWindowsMsvc],
            Self::X86_64PcWindowsMsvc => vec![CliBuildTarget::X86_64PcWindowsMsvc],
        }
    }
}

impl CliBuildTarget {
    pub fn all_targets() -> &'static [Self] {
        &[
            Self::Aarch64AppleDarwin,
            Self::X86_64AppleDarwin,
            Self::Aarch64UnknownLinuxGnu,
            Self::X86_64UnknownLinuxGnu,
            Self::Aarch64PcWindowsMsvc,
            Self::X86_64PcWindowsMsvc,
        ]
    }

    pub fn target_triple(self) -> &'static str {
        match self {
            Self::Aarch64AppleDarwin => "aarch64-apple-darwin",
            Self::X86_64AppleDarwin => "x86_64-apple-darwin",
            Self::Aarch64UnknownLinuxGnu => "aarch64-unknown-linux-gnu",
            Self::X86_64UnknownLinuxGnu => "x86_64-unknown-linux-gnu",
            Self::Aarch64PcWindowsMsvc => "aarch64-pc-windows-msvc",
            Self::X86_64PcWindowsMsvc => "x86_64-pc-windows-msvc",
        }
    }

    pub fn binary_name(self) -> &'static str {
        if self.is_windows() {
            "quizfold.exe"
        } else {
            "quizfold"
        }
    }

    pub fn is_windows(self) -> bool {
        matches!(self, Self::Aarch64PcWindowsMsvc | Self::X86_64PcWindowsMsvc)
    }
}

#[derive(Debug, Subcommand)]
pub(crate) enum CheckCommand {
    Tool,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.command {
            Command::Build { command } => commands::build::run(command),
            Command::Check { command } => commands::check::run(command),
        }
    }
}
