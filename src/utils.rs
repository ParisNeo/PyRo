use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};

/// Create a directory if it doesn't exist
pub fn ensure_dir(path: &Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Write content to a file
pub fn write_file(path: &Path, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Read content from a file
pub fn read_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Join paths in a cross-platform way
pub fn join_paths(base: &str, segments: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(base);
    for segment in segments {
        path.push(segment);
    }
    path
}