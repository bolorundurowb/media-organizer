mod utils;
mod models;

use std::path::Path;

fn main() {
    let directory_path = Path::new("C:\\Users\\bolorundurowb\\Downloads\\tool-sample-media-directory");

    if !directory_path.exists() {
        panic!("Specified source path does not exist");
    }

    let mut directories = vec!();
    let mut files = vec!();

    for entry in std::fs::read_dir(directory_path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read directory entry");

        if entry.path().is_dir() {
            directories.push(entry.path());
        } else {
            files.push(entry.path());
        }
    }

    println!("Found {} files in {:?}", files.len(), directory_path);
    println!("Found {} directories in {:?}", directories.len(), directory_path);
    println!("Directories: {:?}", directories[0]);
    println!("Files: {:?}", files[0]);
}
