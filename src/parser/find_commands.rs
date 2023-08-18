use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

// Finds commands and returns paths to the files that should be moved:
pub fn find_commands(file: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let commands: [&str; 1] = ["\\input"];

    // Open file, get lines:
    let lines = read_lines(file)?;
    let mut files_to_copy: Vec<PathBuf> = Vec::new();

    // Iterate over lines:
    for line in lines {
        if let Ok(line) = line {
            log::info!("got line {}", line);
            for command in commands {
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

// REVIEW: This looks bad, is there a better way to do this?
fn check_line(line: String, command: &str) -> Option<String> {
    match line.contains(command) {
        true => {
            log::info!("line contains command");
            log::info!("Full line: {}", line);
            find_brackets(line)
        }
        false => return None,
    }
}

// REVIEW: Should this be more universal?
fn find_brackets(line: String) -> Option<String> {
    match line.find("{") {
        Some(open) => match line.find("}") {
            Some(close) => {
                let extracted_text = &line[open + 1..close];
                log::info!("Extracted text from line: {}", extracted_text);
                return Some(extracted_text.to_owned());
            }
            _ => return None,
        },
        _ => return None,
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
