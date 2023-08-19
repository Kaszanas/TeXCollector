use std::path::PathBuf;

use clap::Parser;

/// Defines the command line arguments for the TeXCollector
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the .tex file that will be collected.
    #[arg(short, long, default_value = "./files/input/pre_print/main.tex")]
    pub main_file: PathBuf,

    /// Output path where the final collected .tex file will be placed.
    #[arg(short, long, default_value = "./files/output")]
    pub output_path: PathBuf,

    /// Files can either be copied to the output directory
    /// and the paths in the command will be adjusted,
    /// or their contents can be replaced in-place where applicable
    /// [default: false].
    #[arg(short, long, default_value_t = false)]
    pub replace_input: bool,
}
