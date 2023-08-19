use std::{fs, io, path::PathBuf};

pub fn copy_files(from: Vec<PathBuf>, to: PathBuf) -> io::Result<()> {
    for file_path in from {
        fs::copy(file_path, &to)?;
    }

    Ok(())
}
