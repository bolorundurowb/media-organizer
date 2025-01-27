use crate::constants::{METADATA_FILE_NAME, SUBTITLE_FILE_EXTENSION, VIDEO_FILE_EXTENSIONS};
use crate::imdb::get_imdb_result;
use crate::models::MovieMetadata;
use crate::utils::{
    compose_media_name_from_metadata, merge_base_with_file, parse_to_movie_metadata,
};
use inline_colorization::{
    color_blue, color_cyan, color_green, color_magenta, color_reset, color_yellow,
};
use std::fs::DirEntry;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

pub async fn process_directories(directory_entries: Vec<DirEntry>) {
    for directory in directory_entries {
        if needs_processing(&directory.path()) {
            println!(
                "{color_green}Processing directory: {:?}{color_reset}",
                directory.path()
            );
            process_directory(directory).await;
        } else {
            println!(
                "{color_yellow}Skipping directory: {:?}{color_reset}",
                directory.path()
            );
        }
    }

    println!("{color_green}Done processing directories{color_reset}");
}

async fn process_directory(directory_path: DirEntry) {
    let directory_name = directory_path.file_name();
    println!("{color_blue}Processing: {:?}{color_reset}", directory_name);

    if let Some(sub_file_entry) = get_subtitle_entry(&directory_path) {
        println!(
            "{color_cyan}Found subtitle file: {:?}{color_reset}",
            sub_file_entry.path()
        );
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
            println!(
                "{color_green}Copied subtitle to root directory: {:?}{color_reset}",
                target_path
            );
        }
    }

    if let Some(video_file_entry) = get_video_file_entry(&directory_path.path()) {
        println!(
            "{color_magenta}Found video file: {:?}{color_reset}",
            video_file_entry.path()
        );

        let subtitle_entry = get_subtitle_entry(&directory_path);

        delete_except(directory_path.path(), &video_file_entry, &subtitle_entry)
            .expect("Failed to clean movie directory");
        println!("{color_yellow}Cleaned up directory{color_reset}");

        let mut video_file_name = video_file_entry
            .file_name()
            .to_str()
            .map(String::from)
            .unwrap_or_default();

        if directory_name.len() > video_file_name.len() {
            video_file_name = format!(
                "{}.{}",
                directory_name
                    .to_str()
                    .map(String::from)
                    .unwrap_or_default(),
                &video_file_entry
                    .path()
                    .extension()
                    .unwrap()
                    .to_str()
                    .map(String::from)
                    .unwrap_or_default()
            );
        }

        let mut parsed_movie_metadata = parse_to_movie_metadata(&video_file_name);
        let mut composed_file_name = compose_media_name_from_metadata(&parsed_movie_metadata);

        let movie_dest_path = merge_base_with_file(
            &directory_path.path(),
            &format!(
                "{}.{}",
                composed_file_name, &parsed_movie_metadata.file_extension
            ),
        );
        fs::rename(video_file_entry.path(), &movie_dest_path)
            .expect("Failed to rename the movie file");
        println!(
            "{color_green}Renamed movie file to: {:?}{color_reset}",
            movie_dest_path
        );

        if let Some(subtitle) = subtitle_entry {
            let sub_dest_path = merge_base_with_file(
                &directory_path.path(),
                &format!("{}.en.{}", composed_file_name, SUBTITLE_FILE_EXTENSION),
            );
            fs::rename(subtitle.path(), &sub_dest_path)
                .expect("Failed to rename the subtitle file");
            println!(
                "{color_green}Renamed subtitle file to: {:?}{color_reset}",
                sub_dest_path
            );
        } else {
            let imdb_info = get_imdb_result(&composed_file_name).await;

            if let Ok(info) = imdb_info {
                parsed_movie_metadata.media_name = info.title.to_string();
                parsed_movie_metadata.imdb_id = Some(info.id.to_string());
                composed_file_name = compose_media_name_from_metadata(&parsed_movie_metadata);
                println!(
                    "{color_cyan}Updated metadata from IMDb: {:?}{color_reset}",
                    &info
                );
            }
        }

        write_metadata_file(&parsed_movie_metadata, &directory_path.path())
            .expect("Failed to write movie metadata");
        println!("{color_green}Metadata file created{color_reset}");

        let movie_dir_dest_path =
            merge_base_with_file(directory_path.path().parent().unwrap(), &composed_file_name);
        fs::rename(directory_path.path(), &movie_dir_dest_path).expect(
            format!(
                "Failed to rename the movie directory {:?}",
                movie_dir_dest_path
            )
            .as_str(),
        );
        println!(
            "{color_blue}Renamed directory to: {:?}{color_reset}",
            movie_dir_dest_path
        );
    }
}

fn needs_processing(dir_path: &Path) -> bool {
    if has_valid_metadata_json(dir_path) {
        let file_count = fs::read_dir(dir_path)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .count();
        return file_count < 2;
    }

    true
}

fn has_valid_metadata_json(dir_path: &Path) -> bool {
    let metadata_file = dir_path.join(METADATA_FILE_NAME);

    if metadata_file.exists() && metadata_file.is_file() {
        if let Ok(file_content) = fs::read_to_string(&metadata_file) {
            let deserialization_result: serde_json::Result<MovieMetadata> =
                serde_json::from_str(&file_content);
            return deserialization_result.is_ok();
        }
    }

    false
}

fn write_metadata_file(data: &MovieMetadata, directory_path: &Path) -> io::Result<()> {
    // ensure the directory exists
    if !directory_path.exists() {
        fs::create_dir_all(directory_path)?;
    }

    // define the file path
    let file_path = merge_base_with_file(&directory_path, METADATA_FILE_NAME);
    let json_data = serde_json::to_string_pretty(data)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Serialization error: {}", e)))?;

    // write the JSON data to the file
    let mut file = fs::File::create(&file_path)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

fn delete_except<P: AsRef<Path>>(
    dir: P,
    keep: &DirEntry,
    optional_keep: &Option<DirEntry>,
) -> io::Result<()> {
    for entry_result in fs::read_dir(&dir)? {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Failed to read directory entry: {}", err);
                continue;
            }
        };

        let path = entry.path();

        // Check if this is a file/directory to keep
        if path == keep.path() || optional_keep.as_ref().map_or(false, |e| e.path() == path) {
            continue;
        }

        // Attempt to delete directories recursively or files, logging on failure
        let delete_result = if path.is_dir() {
            fs::remove_dir_all(&path)
        } else {
            fs::remove_file(&path)
        };

        if let Err(err) = delete_result {
            eprintln!("Failed to delete {:?}: {}", path, err);
        }
    }

    Ok(())
}

fn get_video_file_entry(dir_path: &Path) -> Option<DirEntry> {
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

    // collect subtitle files in the current directory
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

    // if subtitle files are found, prioritize by name
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

    // recursively search subdirectories for subtitle files
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
