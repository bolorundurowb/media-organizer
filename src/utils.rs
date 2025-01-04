use std::error::Error;
use std::fs::DirEntry;
use std::{fs, io};
use std::path::{Path};
use crate::models::ParsedFileData;

pub fn parse_file_path(file_path: &Path) -> Result<ParsedFileData, Box<dyn Error>> {
    todo!();
}

pub fn get_dir_entry(path: &Path) -> io::Result<DirEntry> {
    // Check if the path exists and is a directory or a file.
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Path not found"));
    }

    // Attempt to read the directory entry. This works for both files and directories.
    fs::read_dir(path.parent().unwrap_or(Path::new(".")))?
        .find(|entry_result| {
            match entry_result {
                Ok(entry) => entry.path() == path,
                Err(_) => false,
            }
        })
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Entry not found in parent directory"))?
        .map_err(|e| e)

}
