use std::{
    io::{self},
    path::PathBuf,
};

use crate::parser::{check_line::check_line, commands::COMMANDS, read_lines};

/// Finds commands and returns paths to the files that should be moved.
fn find_commands(
    lines: io::Lines<io::BufReader<std::fs::File>>,
    commands: &[&'static str],
) -> Result<Vec<PathBuf>, io::Error> {
    // Open file, get lines:
    let mut files_to_copy: Vec<PathBuf> = Vec::new();

    // Iterate over lines:
    for line in lines {
        if let Ok(line) = line {
            log::info!("got line {}", line);
            for command in commands.to_owned() {
                match check_line(line.clone(), command) {
                    Some(str_path) => {
                        let path = PathBuf::from(str_path);
                        let canon_path = path.canonicalize()?;
                        files_to_copy.push(canon_path);
                    }
                    None => {}
                }
            }
        }
    }

    Ok(files_to_copy)
}

/// Runs the parser pipeline
pub fn parser_pipeline(file: PathBuf) -> Result<(), io::Error> {
    let lines = match read_lines::read_lines(file) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };

    let _ = find_commands(lines, &COMMANDS);

    Ok(())
}

pub fn replace_content(lines: io::Lines<io::BufReader<std::fs::File>>) {
    todo!()
}
