use std::path::PathBuf;

use clap::Parser;

/// Defines the command line arguments for the TeXCollector
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CLIArguments {
    /// Path to the .tex file that will be collected.
    #[arg(short, long)]
    pub main_file: PathBuf,

    /// Output path where the final collected .tex file will be placed.
    #[arg(short, long)]
    pub output_path: PathBuf,
}
