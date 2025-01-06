use std::fs::DirEntry;
use crate::constants::{SUBTITLE_FILE_EXTENSION, VIDEO_FILE_EXTENSIONS};

pub fn process_files(file_paths: Vec<DirEntry>) {
    // todo!();
}

fn find_english_subtitle(file_paths: Vec<DirEntry>) -> Option<&DirEntry> {
    let mut subtitle_candidates: Vec<DirEntry> = file_paths
        .into_iter()
        .filter(|entry| {
            if let Some(ext) = entry.path().extension() {
                ext == SUBTITLE_FILE_EXTENSION
            } else {
                false
            }
        })
        .collect();

    // Prioritize subtitles containing "english"
    if let Some(english_subtitle) = subtitle_candidates
        .iter()
        .find(|entry| entry.path().file_name().unwrap_or_default().to_str().map_or(false, |name| name.to_lowercase().contains("english")))
    {
        return Some(english_subtitle.clone());
    }

    // Look for subtitles containing "en"
    if let Some(en_subtitle) = subtitle_candidates
        .iter()
        .find(|entry| entry.path().file_name().unwrap_or_default().to_str().map_or(false, |name| name.to_lowercase().contains("en")))
    {
        return Some(en_subtitle.clone());
    }

    None
}

fn find_files_with_same_prefix(file_paths: Vec<DirEntry>, file_name: &str) -> Vec<DirEntry> {
    file_paths
        .into_iter()
        .filter(|entry| {
            if let Some(file_stem) = entry.path().file_stem() {
                if let Some(stem_str) = file_stem.to_str() {
                    stem_str.starts_with(file_name) && stem_str != file_name
                } else {
                    false
                }
            } else {
                false
            }
        })
        .collect()
}

fn filter_video_files(file_paths: Vec<DirEntry>) -> Vec<DirEntry> {
    file_paths
        .into_iter()
        .filter(|entry| {
            if let Some(ext) = entry.path().extension() {
                if let Some(ext_str) = ext.to_str() {
                    VIDEO_FILE_EXTENSIONS.contains(&ext_str)
                } else {
                    false
                }
            } else {
                false
            }
        })
        .collect()
}
