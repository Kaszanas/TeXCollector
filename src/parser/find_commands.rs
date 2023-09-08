use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader, Lines},
    path::PathBuf,
};

use crate::parser::commands::COMMANDS;
use crate::parser::read_lines;

// TODO: No need for HashMap here, just return at which keys are the lines that are with commands:
/// Finds commands and returns paths to the files that should be moved.
fn find_commands(
    lines: Lines<BufReader<File>>,
    commands: &[&'static str],
) -> Result<HashMap<usize, String>, io::Error> {
    // Initializing empty vector to populate with results:
    let mut lines_commands = HashMap::new();

    // Iterate over lines, find the commands and their line numbers:
    for (index, line) in lines.enumerate() {
        let check_line = line?;
        log::info!("got line {}", check_line);

        commands
            .iter()
            .filter(|&&cmd| check_line.contains(cmd))
            .for_each(|_| {
                lines_commands.insert(index, check_line.clone());
            });
    }

    Ok(lines_commands)
}

/// Runs the parser pipeline
pub fn parser_pipeline(file: PathBuf, replace_input: bool) -> Result<(), io::Error> {
    // Open the file and get the lines:
    let lines = read_lines::read_lines(file)?;

    // TODO: Hash the lines for faster lookup and replacement:
    let hashed_lines = read_lines::hash_lines(lines);

    // Find commands and their line numbers:
    let found_commands = find_commands(lines, &COMMANDS)?;

    // TODO: if command is of type input, then you can either replace
    // the whole line with the content of the file.
    // Or you can replace it with flattened path to the file.
    replace_content(hashed_lines, found_commands, replace_input);

    Ok(())
}

fn replace_content(
    hashed_lines: HashMap<usize, String>,
    found_commands: HashMap<usize, String>,
    replace_input: bool,
) -> Result<(), io::Error> {
    for (index, command) in found_commands {
        todo!()
    }

    Ok(())
}
