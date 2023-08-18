mod cli;
mod files;
mod parser;

use clap::Parser;
use simple_logger::SimpleLogger;

use crate::parser::{find_commands, read_lines};

fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("Initialized the program!");

    // Parse command-line arguments:
    let args = cli::Cli::parse();
    let main_file = &args.main_file;
    log::info!("Set LaTeX main file to: {}", main_file.to_string_lossy());
    let output_path = &args.output_path;
    log::info!(
        "Set output path for the collected files to: {}",
        output_path.to_string_lossy()
    );

    // Parse the main .tex file.
    // TODO: Error handling!

    let files = find_commands::parser_pipeline(main_file.to_path_buf()).unwrap();
    log::info!("Got files {:#?}", files);
}
