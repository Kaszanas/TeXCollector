use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

// REVIEW: This looks bad, is there a better way to do this?
pub fn check_line(line: String, command: &str) -> Option<String> {
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
pub fn find_brackets(line: String) -> Option<String> {
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
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
