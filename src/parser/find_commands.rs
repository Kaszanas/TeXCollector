use std::{
    io::{self},
    path::PathBuf,
};

use crate::parser::utils;

// Finds commands and returns paths to the files that should be moved:
fn find_commands(
    lines: io::Lines<io::BufReader<std::fs::File>>,
    commands: Vec<&str>,
) -> Result<Vec<PathBuf>, io::Error> {
    // Open file, get lines:
    let mut files_to_copy: Vec<PathBuf> = Vec::new();

    // Iterate over lines:
    for line in lines {
        if let Ok(line) = line {
            log::info!("got line {}", line);
            for command in commands.to_owned() {
                match utils::check_line(line.clone(), command) {
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

pub fn parser_pipeline(file: PathBuf) -> Result<(), io::Error> {
    let commands = ["\\input"].to_vec();

    let lines = match utils::read_lines(file) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };

    find_commands(lines, commands);

    Ok(())
}

pub fn replace_content(lines: io::Lines<io::BufReader<std::fs::File>>) {
    todo!()
}
