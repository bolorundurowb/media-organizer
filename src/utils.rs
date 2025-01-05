use std::error::Error;
use std::fs::DirEntry;
use std::{fs, io};
use std::path::{Path};
use crate::models::{MovieMetadata, ParsedMovieFileData};

pub fn parse_to_movie_metadata(file_name: &str) -> Result<MovieMetadata, Box<dyn Error>> {
    // todo!();
   Ok(MovieMetadata {
       original_file_name: "".to_string(),
       media_name: "".to_string(),
       release_year: None,
       encoding_format: None,
       resolution: None,
       additional_data: vec![],
       file_extension: "".to_string(),
   })
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
