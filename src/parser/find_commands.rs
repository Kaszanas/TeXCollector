use std::{
    collections::HashMap,
    io::{self},
    path::PathBuf,
};

use crate::parser::{check_line::check_line, commands::COMMANDS, read_lines};

/// Finds commands and returns paths to the files that should be moved.
fn find_commands(
    lines: io::Lines<io::BufReader<std::fs::File>>,
    commands: &[&'static str],
) -> Result<Vec<(usize, String)>, io::Error> {
    // Initializing empty vector to populate with results:
    let mut lines_commands: Vec<(usize, String)> = Vec::new();

    // Iterate over lines, find the commands and their line numbers:
    for (index, line) in lines.enumerate() {
        if let Ok(line) = line {
            log::info!("got line {}", line);
            for command in commands.to_owned() {
                match check_line(line.clone(), command) {
                    Some(found_line) => {
                        lines_commands.push((index, found_line));
                    }
                    None => {}
                }
            }
        }
    }

    Ok(lines_commands)
}

/// Runs the parser pipeline
pub fn parser_pipeline(file: PathBuf, replace_input: bool) -> Result<(), io::Error> {
    // Open the file and get the lines:
    let lines = match read_lines::read_lines(file) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };

    // TODO: Hash the lines for faster lookup and replacement:
    let hashed_lines = read_lines::hash_lines(lines);

    // // Find commands and their line numbers:
    // let found_commands = match find_commands(lines, &COMMANDS) {
    //     Ok(commands) => commands,
    //     Err(err) => return Err(err),
    // };

    // // TODO: if command is of type input, then you can either replace
    // // the whole line with the content of the file.
    // // Or you can replace it with flattened path to the file.
    // replace_content(&lines, found_commands, replace_input);

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
