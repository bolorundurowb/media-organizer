use crate::constants::{SUBTITLE_FILE_EXTENSION, VIDEO_FILE_EXTENSIONS};
use std::{fs, io};
use std::fs::DirEntry;
use std::path::Path;
use crate::utils::get_dir_entry;

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

    // if a subtitle is found and is in a nested directory then it should be moved into the root dir
    if let Some(sub_file_entry) = get_subtitle_entry(&dir_entries) {
        let subtitle_exists_in_movie_dir = dir_entries.iter().any(|entry| entry.path() == sub_file_entry.path());

        if !subtitle_exists_in_movie_dir {
            let target_path = directory_path.path().join(sub_file_entry.file_name());
            fs::copy(sub_file_entry.path(), &target_path).expect("Failed to copy subtitle file to root directory");
        }
    }

   if let Some(movie_file_entry) = get_movie_entry(&dir_entries) {
        // delete every file except the movie and its subtitle
       delete_except(directory_path.path(), movie_file_entry, sub_file_entry_opt).expect("Failed to clean movie directory");

    }

    println!("Processing directory {:?}", dir_entries);
}

fn delete_except<P: AsRef<Path>>(
    dir: P,
    keep: &DirEntry,
    optional_keep: Option<&DirEntry>,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // Skip the files or directories to keep
        if entry.path() == keep.path() || optional_keep.map_or(false, |e| e.path() == path) {
            continue;
        }

        // Recursively delete directories
        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }

    Ok(())
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

fn get_subtitle_entry(dir_entries: &Vec<DirEntry>) -> Option<&DirEntry> {
    let mut entries: Vec<&DirEntry> = Vec::new();

    // Recursively collect all subtitle files
    fn collect_subtitle_files(dir_entries: &Vec<DirEntry>, entries: &mut Vec<&DirEntry>) {
        if !dir_entries.is_empty() {
            for dir_entry in dir_entries {
                 if dir_entry.path().is_dir() {
                    let sub_dir_entries: Vec<DirEntry> = fs::read_dir(dir_entry.path())
                        .expect("Failed to read movie sub directory")
                        .map(|entry| entry.expect("Failed to read movie sub directory"))
                        .collect();
                    collect_subtitle_files(&sub_dir_entries, entries);
                } else if dir_entry.path().extension().unwrap().to_str().unwrap() == SUBTITLE_FILE_EXTENSION {
                    entries.push(dir_entry);
                }
            }
        }
    }

    collect_subtitle_files(dir_entries, &mut entries);

    if entries.is_empty() {
        return None;
    }

    // Prioritize files with "english" in their name
    let english_matches: Vec<&DirEntry> = entries
        .iter()
        .filter(|path| {
            path.file_name()
                .to_string_lossy()
                .to_lowercase()
                .contains("english")
        })
        .cloned()
        .collect();

    if !english_matches.is_empty() {
        return Some(english_matches[0]);
    }

    // Then prioritize files with "en" in their name
    let en_matches: Vec<&DirEntry> = entries
        .iter()
        .filter(|path| {
            path.file_name()
                .to_string_lossy()
                .to_lowercase()
                .contains("en")
        })
        .cloned()
        .collect();

    if !en_matches.is_empty() {
        return Some(en_matches[0]);
    }

    // If no matches are found, return the first file
    Some(entries[0].clone())
}
