use crate::constants::{SUBTITLE_FILE_EXTENSION, VIDEO_FILE_EXTENSIONS};
use crate::utils::{
    format_movie_metadata, get_raw_file_name_and_extension, merge_base_with_file,
    parse_to_movie_metadata,
};
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

pub fn process_files(directory_path: &Path, file_paths: Vec<DirEntry>) {
    let video_file_entries = filter_video_files(&file_paths);
    for video_file_entry in &video_file_entries {
        let mut subtitle_entry: Option<&DirEntry> = None;
        let related_files = find_files_with_same_prefix(
            &file_paths,
            video_file_entry.file_name().to_str().unwrap(),
        );

        if related_files.len() > 0 {
            for related_file in related_files {
                // if it is not a sub, then delete it
                if is_subtitle_file(&related_file) {
                    subtitle_entry = Some(related_file);
                } else {
                    fs::remove_file(&related_file.path())
                        .expect("Failed to delete unrelated media file");
                }
            }
        }

        let video_file_name = video_file_entry
            .file_name()
            .to_str()
            .map(String::from)
            .unwrap_or_default();
        let parsed_movie_metadata = parse_to_movie_metadata(&video_file_name);
        let composed_file_name = format_movie_metadata(&parsed_movie_metadata);

        // create the movie directory
        let movie_directory_path = merge_base_with_file(&directory_path, &composed_file_name);
        fs::create_dir(&movie_directory_path).expect("Failed to create the movie directory");

        // rename the files
        let movie_dest_path = merge_base_with_file(
            Path::new(&movie_directory_path),
            &format!(
                "{}.{}",
                composed_file_name, &parsed_movie_metadata.file_extension
            ),
        );
        fs::rename(video_file_entry.path(), movie_dest_path)
            .expect("Failed to rename the movie file");

        if subtitle_entry.is_some() {
            let sub_dest_path = merge_base_with_file(
                Path::new(&movie_directory_path),
                &format!("{}.{}", composed_file_name, SUBTITLE_FILE_EXTENSION),
            );
            fs::rename(subtitle_entry.unwrap().path(), sub_dest_path)
                .expect("Failed to rename the subtitle file");
        }
    }
}

fn is_subtitle_file(entry: &DirEntry) -> bool {
    entry
        .path()
        .extension()
        .and_then(|ext| ext.to_str())
        .map_or(false, |ext| ext == SUBTITLE_FILE_EXTENSION)
}

fn find_english_subtitle(file_paths: &[DirEntry]) -> Option<&DirEntry> {
    let subtitle_candidates: Vec<&DirEntry> = file_paths
        .iter()
        .filter(|entry| {
            if let Some(ext) = entry.path().extension() {
                ext == SUBTITLE_FILE_EXTENSION
            } else {
                false
            }
        })
        .collect();

    // Prioritize subtitles containing "english"
    if let Some(english_subtitle) = subtitle_candidates.iter().find(|entry| {
        entry
            .path()
            .file_name()
            .unwrap_or_default()
            .to_str()
            .map_or(false, |name| name.to_lowercase().contains("english"))
    }) {
        return Some(english_subtitle);
    }

    // Look for subtitles containing "en"
    if let Some(en_subtitle) = subtitle_candidates.iter().find(|entry| {
        entry
            .path()
            .file_name()
            .unwrap_or_default()
            .to_str()
            .map_or(false, |name| name.to_lowercase().contains("en"))
    }) {
        return Some(en_subtitle);
    }

    None
}

fn find_files_with_same_prefix<'a>(
    file_paths: &'a [DirEntry],
    file_name: &str,
) -> Vec<&'a DirEntry> {
    let (raw_file_name, _) = get_raw_file_name_and_extension(&file_name);
    file_paths
        .iter()
        .filter_map(|entry| {
            if let Some(file_stem) = entry.path().file_stem() {
                if let Some(stem_str) = file_stem.to_str() {
                    if stem_str.starts_with(raw_file_name) && stem_str != raw_file_name {
                        return Some(entry);
                    }
                }
            }
            None
        })
        .collect()
}

fn filter_video_files(file_paths: &[DirEntry]) -> Vec<&DirEntry> {
    file_paths
        .iter()
        .filter_map(|entry| {
            if let Some(ext) = entry.path().extension() {
                if let Some(ext_str) = ext.to_str() {
                    if VIDEO_FILE_EXTENSIONS.contains(&ext_str) {
                        return Some(entry);
                    }
                }
            }
            None
        })
        .collect()
}
