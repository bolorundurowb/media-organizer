mod constants;
mod imdb;
mod models;
mod processors;
mod subtitles;
mod utils;

use std::fs;
use std::fs::DirEntry;
use std::path::Path;

#[tokio::main]
async fn main() {
    let directory_path =
        // Path::new("C:\\Users\\bolorundurowb\\Downloads\\tool-sample-media-directory");
        Path::new("\\\\bolorundurology\\Media\\Movies");

    if !directory_path.exists() {
        panic!("Specified source path does not exist");
    }

    let files: Vec<DirEntry> = fs::read_dir(&directory_path)
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().is_file())
        .collect();

    // collect this into directories
    if files.len() > 0 {
        processors::files::process_files(&directory_path, files);
    }

    let directories = fs::read_dir(&directory_path)
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().is_dir())
        .collect();
    processors::directories::process_directories(directories).await;
}
