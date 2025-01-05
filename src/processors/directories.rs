use crate::constants::{SUBTITLE_FILE_EXTENSION, VIDEO_FILE_EXTENSIONS};
use crate::utils::{get_dir_entry, parse_to_movie_metadata};
use std::fs::DirEntry;
use std::path::Path;
use std::{fs, io};

pub fn process_directories(directory_paths: Vec<DirEntry>) {
    for directory in directory_paths {
        process_directory(directory);
    }
}

fn process_directory(directory_path: DirEntry) {
    let directory_name = directory_path.file_name();

    // if a subtitle is found and is in a nested directory then it should be moved into the root dir
    if let Some(sub_file_entry) = get_subtitle_entry(&directory_path) {
        let subtitle_exists_in_movie_dir = fs::read_dir(directory_path.path())
            .ok()
            .and_then(|entries| {
                Some(
                    entries
                        .filter_map(Result::ok)
                        .any(|entry| entry.path() == sub_file_entry.path()),
                )
            })
            .unwrap_or(false);

        if !subtitle_exists_in_movie_dir {
            let target_path = directory_path.path().join(sub_file_entry.file_name());
            fs::copy(sub_file_entry.path(), &target_path)
                .expect("Failed to copy subtitle file to root directory");
        }
    }

    if let Some(movie_file_entry) = get_movie_entry(&directory_path.path()) {
        let subtitle_entry = get_subtitle_entry(&directory_path);

        // delete every file except the movie and its subtitle
        // delete_except(directory_path.path(), &movie_file_entry, &subtitle_entry)
        //     .expect("Failed to clean movie directory");

        // determine name to be parsed
        let mut movie_file_name = movie_file_entry.file_name().to_str().map(String::from).unwrap_or_default();

        if directory_name.len() > movie_file_name.len() {
            movie_file_name = directory_name.to_str().map(String::from).unwrap_or_default();
        }

        let parsed_movie_metadata = parse_to_movie_metadata(&movie_file_name).expect("Failed to parse movie metadata");
        println!("Movie file name: {}, Metadata: {:?}", movie_file_name, parsed_movie_metadata);

        // let unified_file_name =
    }
}

fn delete_except<P: AsRef<Path>>(
    dir: P,
    keep: &DirEntry,
    optional_keep: &Option<DirEntry>,
) -> io::Result<()> {
    for entry_result in fs::read_dir(&dir)? {
        let entry = entry_result?;
        let path = entry.path();

        // Check if this is a file/directory to keep
        if path == keep.path() || optional_keep.as_ref().map_or(false, |e| e.path() == path) {
            continue;
        }

        // Delete directories recursively or files
        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}

fn get_movie_entry(dir_path: &Path) -> Option<DirEntry> {
    fs::read_dir(dir_path)
        .ok()?
        .filter_map(|entry| entry.ok())
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

fn get_subtitle_entry(dir_entry: &DirEntry) -> Option<DirEntry> {
    let dir_entries = fs::read_dir(dir_entry.path()).ok()?;

    // Collect subtitle files in the current directory
    let mut subtitle_files: Vec<DirEntry> = dir_entries
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.path().is_file()
                && entry
                    .path()
                    .extension()
                    .map(|ext| ext == SUBTITLE_FILE_EXTENSION)
                    .unwrap_or(false)
        })
        .collect();

    // If subtitle files are found, prioritize by name
    if !subtitle_files.is_empty() {
        subtitle_files.sort_by_key(|entry| {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            (
                !name.contains("english"),
                !name.contains("en"),
                name.clone(),
            )
        });
        return subtitle_files.into_iter().next(); // Return the first (best match)
    }

    // Recursively search subdirectories for subtitle files
    let dir_entries = fs::read_dir(dir_entry.path()).ok()?;
    for entry in dir_entries.filter_map(Result::ok) {
        if entry.path().is_dir() {
            if let Some(subtitle) = get_subtitle_entry(&entry) {
                return Some(subtitle);
            }
        }
    }

    None
}
