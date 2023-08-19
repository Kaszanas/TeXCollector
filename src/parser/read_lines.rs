use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn hash_lines(lines: io::Lines<io::BufReader<File>>) -> HashMap<usize, String> {
    let mut res = HashMap::new();

    for (index, line) in lines.enumerate() {
        if let Ok(ok_line) = line {
            res.insert(index, ok_line);
        }
    }

    res
}
