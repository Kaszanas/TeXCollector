use std::path::PathBuf;

use clap::Parser;

/// Defines the command line arguments for the TeXCollector
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the .tex file that will be collected.
    #[arg(short, long, default_value = "main.tex")]
    pub main_file: PathBuf,

    /// Output path where the final collected .tex file will be placed.
    #[arg(short, long, default_value = "./output")]
    pub output_path: PathBuf,

    /// files can either be copied, or their contents can be replaced in-place [default: false].
    #[arg(short, long, default_value_t = false)]
    pub copy: bool,
}
