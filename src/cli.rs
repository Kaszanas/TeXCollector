use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = "main.tex")]
    pub main_file: PathBuf,

    #[arg(short, long, default_value = "./output")]
    pub output_path: PathBuf,
}
