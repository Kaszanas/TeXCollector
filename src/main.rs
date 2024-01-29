use clap::Parser;
use simple_logger::SimpleLogger;
use texcollector::{cli, lexer, parser};

fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("Initialized the program!");

    // Parse command-line arguments:
    let args = cli::CLIArguments::parse();
    let main_file = &args.main_file;
    log::info!("Set LaTeX main file to: {}", main_file.to_string_lossy());
    let output_path = &args.output_path;
    log::info!(
        "Set output path for the collected files to: {}",
        output_path.to_string_lossy()
    );
    let replace_input = &args.replace_input;
}
