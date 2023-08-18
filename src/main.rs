mod cli;
mod files;
mod parser;

use clap::Parser;
use simple_logger::SimpleLogger;

use crate::parser::find_commands::find_commands;

fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("Initialized the program!");

    // Parse command-line arguments:
    let args = cli::Cli::parse();
    let main_file = &args.main_file;
    log::info!("Main file set to: {}", main_file.to_string_lossy());

    // Parse the main .tex file.
    // TODO: Error handling!
    let files = find_commands(main_file.to_owned()).unwrap();
    log::info!("Got files {:#?}", files);
}
