use crate::constants::{SUBTITLE_FILE_EXTENSION, VIDEO_FILE_EXTENSIONS};
use std::fs;
use std::fs::DirEntry;

pub fn process_directories(directory_paths: Vec<DirEntry>) {
    for directory in directory_paths {
        process_directory(directory);
    }
}

fn process_directory(directory_path: DirEntry) {
    let directory_name = directory_path.file_name();
    let mut dir_entries = vec![];

    for entry in fs::read_dir(directory_path.path()).expect("Failed to read movie sub directory") {
        let entry = entry.expect("Failed to read movie sub directory");
        dir_entries.push(entry);
    }

    println!("Processing directory {:?}", dir_entries);
}

fn get_movie_entry(entries: &Vec<DirEntry>) -> Option<&DirEntry> {
    entries
        .iter()
        .filter(|entry| {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                VIDEO_FILE_EXTENSIONS.contains(&ext.to_string_lossy().to_lowercase().as_str())
            } else {
                false
            }
        })
        .max_by_key(|entry| entry.metadata().map(|meta| meta.len()).unwrap_or(0))
}

fn get_sub_entry(dir_entries: &Vec<DirEntry>) -> Option<&DirEntry> {
    let mut entries: Vec<&DirEntry> = Vec::new();

    // Recursively collect all subtitle files
    fn collect_subtitle_files(dir_entries: &Vec<DirEntry>, entries: &mut Vec<&DirEntry>) {
        if !dir_entries.is_empty() {
            for dir_entry in dir_entries {
                if dir_entry.path().is_file() {
                    entries.push(dir_entry);
                } else if dir_entry.path().is_dir() {
                    let sub_dir_entries: Vec<DirEntry> = fs::read_dir(dir_entry.path())
                        .expect("Failed to read movie sub directory")
                        .map(|entry| entry.expect("Failed to read movie sub directory"))
                        .collect();
                    collect_subtitle_files(&sub_dir_entries, entries);
                }
            }
        }
    }

    collect_subtitle_files(dir_entries, &mut entries);

    if entries.is_empty() {
        return None;
    }

    // Prioritize files with "english" in their name
    let english_matches: Vec<DirEntry> = entries
        .iter()
        .filter(|path| {
            path.file_name()?
                .to_string_lossy()
                .to_lowercase()
                .contains("english")
        })
        .cloned()
        .collect();

    if !english_matches.is_empty() {
        return Some(english_matches[0].cloned());
    }

    // Then prioritize files with "en" in their name
    let en_matches: Vec<DirEntry> = entries
        .iter()
        .filter(|path| {
            path.file_name()?
                .to_string_lossy()
                .to_lowercase()
                .contains("en")
        })
        .cloned()
        .collect();

    if !en_matches.is_empty() {
        return Some(en_matches[0].cloned());
    }

    // If no matches are found, return the first file
    Some(entries[0].clone())
}
