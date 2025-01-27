use crate::tv_shows_processors;
use inline_colorization::{color_red, color_reset, color_yellow};
use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::path::{Path, PathBuf};

pub async fn handle_tv_shows(dir_path: &Path, dir_entries: ReadDir) {
    let directories: Vec<DirEntry> = dir_entries
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().is_dir())
        .collect();

    if directories.len() == 0 {
        println!("{color_yellow}No TV show directories to process{color_reset}");
        return;
    }

    delete_excluded_files(dir_path, &directories).expect("Failed to delete files");

    tv_shows_processors::directories::process_directories(directories).await;
}

fn delete_excluded_files(dir_path: &Path, included_entries: &Vec<DirEntry>) -> std::io::Result<()> {
    let included_paths: Vec<PathBuf> = included_entries.iter().map(|entry| entry.path()).collect();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if !included_paths.contains(&path) {
            if path.is_dir() {
                println!("{color_red}Deleting directory path: {path:?}{color_reset}");
                fs::remove_dir_all(&path)?;
            } else {
                println!("{color_red}Deleting file: {path:?}{color_reset}");
                fs::remove_file(&path)?;
            }
        }
    }

    Ok(())
}
