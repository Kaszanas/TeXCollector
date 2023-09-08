use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines(filename: impl AsRef<Path>) -> io::Result<Lines<BufReader<File>>> {
    let lines = BufReader::new(File::open(filename)?).lines();
    Ok(lines)
}

pub fn hash_lines(lines: Lines<BufReader<File>>) -> HashMap<usize, String> {
    let mut res = HashMap::new();

    for (index, line) in lines.enumerate() {
        if let Ok(ok_line) = line {
            res.insert(index, ok_line);
        }
    }

    res
}
