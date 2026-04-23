use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "A DSL compiler for ERC-20 token contracts", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Compile a .tc file to EVM bytecode
    Build {
        /// Input .tc file
        file: PathBuf,

        /// Output as hex to stdout
        #[arg(long)]
        hex: bool,

        /// Output file (default: <input>.bin)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Show verbose compilation steps
        #[arg(long)]
        verbose: bool,
    },

    /// Check for errors and warnings without compiling
    Lint {
        /// Input .tc file
        file: PathBuf,
    },

    /// Interactive wizard to generate a .tc file
    Init,
}
