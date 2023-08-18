use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

pub fn find_commands(file: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let commands: [&str; 1] = ["/input"];

    // Open file, get lines:
    let lines = read_lines(file)?;

    let mut files_to_copy: Vec<PathBuf> = Vec::new();

    // REVIEW: This looks and probably works like shit.
    // REVIEW: Overall look at the complexity of this code.
    // REVIEW: What is this Flutter?
    // Iterate over lines:
    for line in lines {
        if let Ok(line) = line {
            log::info!("got line");
            for command in commands {
                if line.contains(command) {
                    log::info!("line contains command");
                    log::info!("Full line: {}", line);
                    // REVIEW: Dafuq is that?
                    if let Some(open) = command.find("{") {
                        if let Some(close) = command.find("}") {
                            let extracted_text = &command[open + 1..close];
                            log::info!("Extracted text from line: {}", extracted_text);
                            files_to_copy.push(PathBuf::from(extracted_text));
                        }
                    }
                }
            }
        }
    }

    Ok(files_to_copy)
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
