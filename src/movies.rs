use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::path::Path;
use inline_colorization::{color_yellow, color_reset};
use crate::movie_processors;

pub async fn handle_movies(directory_path: &Path, dir_entries: ReadDir) {
    let files: Vec<DirEntry> = dir_entries
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().is_file())
        .collect();

    // collect this into directories
    if files.len() > 0 {
        movie_processors::files::process_files(&directory_path, files);
    } else {
        println!("{color_yellow}No files to process{color_reset}")
    }

    let directories = fs::read_dir(&directory_path)
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().is_dir())
        .collect();
    movie_processors::directories::process_directories(directories).await;
}