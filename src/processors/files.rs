use crate::constants::VIDEO_FILE_EXTENSIONS;
use crate::utils::get_raw_file_name_and_extension;
use inline_colorization::{color_red, color_green, color_reset};
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

pub fn process_files(directory_path: &Path, file_paths: Vec<DirEntry>) {
    let video_file_entries = filter_video_files(&file_paths);
    for video_file_entry in &video_file_entries {
        let raw_video_file_name = &video_file_entry
            .path()
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(String::from)
            .unwrap_or_default();

        // Create the movie directory
        let movie_directory_path = directory_path.join(raw_video_file_name);
        if let Err(e) = fs::create_dir(&movie_directory_path) {
            eprintln!(
                "{color_red}Failed to create the movie sub-directory: {}{color_reset}",
                e
            );
            continue;
        }

        // Move the video file to the sub-directory
        let movie_dest_path = movie_directory_path.join(video_file_entry.file_name());
        if let Err(e) = fs::rename(video_file_entry.path(), &movie_dest_path) {
            eprintln!(
                "{color_red}Failed to move the movie file to the sub-directory: {}{color_reset}",
                e
            );
            continue;
        }

        println!(
            "{color_green}Successfully moved video file: {}{color_reset}",
            raw_video_file_name
        );

        // Find and move related files
        let related_files = find_files_with_same_prefix(
            &file_paths,
            video_file_entry.file_name().to_str().unwrap(),
        );
        for related_file_entry in related_files {
            let related_dest_path = movie_directory_path.join(
                related_file_entry
                    .file_name()
                    .to_str()
                    .map(String::from)
                    .unwrap_or_default(),
            );
            if let Err(e) = fs::rename(related_file_entry.path(), &related_dest_path) {
                eprintln!("{color_red}Failed to move a related file to the sub-directory: {}{color_reset}", e);
            } else {
                println!(
                    "{color_green}Successfully moved related file: {}{color_reset}",
                    related_file_entry.file_name().to_string_lossy()
                );
            }
        }
    }
}

fn find_files_with_same_prefix<'a>(
    file_paths: &'a [DirEntry],
    file_name: &str,
) -> Vec<&'a DirEntry> {
    let (raw_file_name, _) = get_raw_file_name_and_extension(file_name);
    file_paths
        .iter()
        .filter(|entry| {
            entry
                .path()
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map_or(false, |stem_str| {
                    stem_str.starts_with(raw_file_name) && stem_str != raw_file_name
                })
        })
        .collect()
}

fn filter_video_files(file_paths: &[DirEntry]) -> Vec<&DirEntry> {
    file_paths
        .iter()
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map_or(false, |ext| VIDEO_FILE_EXTENSIONS.contains(&ext))
        })
        .collect()
}
