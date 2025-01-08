use crate::constants::{SUBTITLE_FILE_EXTENSION, VIDEO_FILE_EXTENSIONS};
use crate::utils::get_raw_file_name_and_extension;
use std::fs::DirEntry;

pub fn process_files(file_paths: Vec<DirEntry>) {
    let video_files = filter_video_files(&file_paths);
    for video_file in &video_files {
        let related_files =
            find_files_with_same_prefix(&file_paths, video_file.file_name().to_str().unwrap());
        println!(
            "Found {} related files for {:?}: {:?}",
            related_files.len(),
            video_file.file_name(),
            &related_files
        );

        if let Some(subtitle) = find_english_subtitle(&file_paths) {
            println!("Found subtitle: {:?}", subtitle.file_name());
        } else {
            println!(
                "No matching subtitle found for {:?}",
                video_file.file_name()
            );
        }
    }
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
