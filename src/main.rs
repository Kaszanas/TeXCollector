mod cli;
mod parser;

use clap::Parser;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("Initialized the program!");

    let args = cli::Cli::parse();

    let main_file = &args.main_file;
    // TODO: Parse command-line arguments:

    // TODO: Parse the main .tex file
}
