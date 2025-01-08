mod utils;
mod models;
mod processors;
mod constants;

use std::fs;
use std::path::Path;

fn main() {
    let directory_path = Path::new("C:\\Users\\bolorundurowb\\Downloads\\tool-sample-media-directory");

    if !directory_path.exists() {
        panic!("Specified source path does not exist");
    }

    let mut directories = vec!();
    let mut files = vec!();

    for entry in fs::read_dir(directory_path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read directory entry");

        if entry.path().is_dir() {
            directories.push(entry);
        } else {
            files.push(entry);
        }
    }

    processors::directories::process_directories(directories);
    processors::files::process_files(&directory_path, files);
}
