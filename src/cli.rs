use std::path::PathBuf;

use clap::Parser;

/// Defines the command line arguments for the TeXCollector
#[derive(Parser, Debug)]
#[command(
    author = "Andrzej 'Kaszanas' Białecki",
    version,
    about = "TeXCollector helps you with bigger LaTeX projects spanning multiple files. Assumes existing latexpand installation.",
    long_about = "TeXCollector helps you with bigger LaTeX projects spanning multiple files. Assumes existing latexpand installation. TeXCollector inlines inputs, copies figures, .sty and .bib files, and emits a single submission-ready .tex."
)]
pub struct CLIArguments {
    /// Path to the .tex file that will be collected.
    #[arg(
        short,
        long,
        value_name = "MAIN_TEX",
        help = "Main .tex file that will be used as the entry point for the collection process."
    )]
    pub input_file: PathBuf,

    /// Output path where the final collected .tex file will be placed.
    #[arg(
        short,
        long,
        value_name = "OUTPUT_DIR",
        help = "Directory where the final collected .tex file will be placed."
    )]
    pub output_path: PathBuf,

    #[arg(
        short = 'f',
        long,
        value_name = "OUTPUT_FILE_NAME",
        default_value = "collected.tex",
        help = "Optional name for the output .tex file. If not provided, the default name will be used."
    )]
    pub output_file_name: Option<String>,
}
