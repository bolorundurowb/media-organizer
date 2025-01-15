use crate::models::{MediaEncodingFormat, MovieMetadata};
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC};
use regex::Regex;
use std::path::Path;

pub fn parse_to_movie_metadata(file_name: &str) -> MovieMetadata {
    let (raw_file_name, file_extension) = get_raw_file_name_and_extension(file_name);
    compose_movie_metadata(raw_file_name, file_extension)
}

pub fn merge_base_with_file(base_path: &Path, file_name: &str) -> String {
    let merged_path = base_path.join(file_name);
    merged_path.to_string_lossy().into_owned()
}

pub fn compose_media_name_from_metadata(metadata: &MovieMetadata) -> String {
    let mut result = metadata.media_name.clone();

    if let Some(year) = metadata.release_year {
        result.push_str(&format!(" ({})", year));
    }

    if let Some(resolution) = metadata.resolution {
        result.push_str(&format!(" [{}p]", resolution));
    }

    clean_filename(&result).unwrap()
}

pub fn get_raw_file_name_and_extension(file_name: &str) -> (&str, String) {
    if let Some(pos) = file_name.rfind('.') {
        let name = &file_name[..pos];
        let extension = file_name[pos + 1..].to_string();
        (name, extension)
    } else {
        (file_name, String::new())
    }
}

pub fn url_encode(input: &str) -> String {
    // Define the character set to *not* encode. This is the "unreserved" set from RFC 3986.
    const UNRESERVED: AsciiSet = NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'.')
        .remove(b'_')
        .remove(b'~');

    percent_encoding::percent_encode(input.as_bytes(), &UNRESERVED).to_string()
}

fn compose_movie_metadata(raw_file_name: &str, file_extension: String) -> MovieMetadata {
    let sanitized_file_name = raw_file_name.replace('.', " ");
    let parts: Vec<&str> = sanitized_file_name.split_whitespace().collect();
    let original_file_name = raw_file_name.to_string();

    // Regular expressions for extracting metadata
    let year_re = Regex::new(r"^(19|20)\d{2}$").unwrap();
    let resolution_re = Regex::new(r"^(4K|720p|1080p|2160p)$").unwrap();
    let encoding_format_re = Regex::new(r"(?i)(x264|x265|h264|h265)").unwrap();

    let mut release_year = None;
    let mut resolution = None;
    let mut encoding_format = None;
    let mut additional_data = Vec::new();
    let mut media_name_parts = Vec::new();
    let mut metadata_started = false;

    for part in parts {
        if !metadata_started {
            // Check if part matches any metadata regex
            if year_re.is_match(part) {
                release_year = part.parse::<u16>().ok();
                metadata_started = true;
            } else if resolution_re.is_match(part) {
                resolution = match part {
                    "4K" | "2160p" => Some(2160),
                    "1080p" => Some(1080),
                    "720p" => Some(720),
                    _ => None,
                };
                metadata_started = true;
            } else if encoding_format_re.is_match(part) {
                encoding_format = MediaEncodingFormat::from(part);
                metadata_started = true;
            }

            if !metadata_started {
                media_name_parts.push(part.to_string());
            }
        } else {
            // Process metadata or add to additional_data
            if year_re.is_match(part) {
                release_year = part.parse::<u16>().ok();
            } else if resolution_re.is_match(part) {
                resolution = match part {
                    "4K" | "2160p" => Some(2160),
                    "1080p" => Some(1080),
                    "720p" => Some(720),
                    _ => None,
                };
            } else if encoding_format_re.is_match(part) {
                encoding_format = MediaEncodingFormat::from(part);
            } else {
                additional_data.push(part.to_string());
            }
        }
    }

    let media_name = media_name_parts.join(" ");

    MovieMetadata {
        original_file_name,
        media_name: to_title_case(&media_name),
        release_year,
        encoding_format,
        resolution,
        additional_data,
        file_extension,
        imdb_id: None,
    }
}

fn to_title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn clean_filename(filename: &str) -> Option<String> {
    // Reserved Windows names (case-insensitive)
    let reserved_names = [
        "CON", "PRN", "AUX", "NUL",
        "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
        "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    // Invalid characters for both Windows and Linux
    let invalid_chars: &[char] = &['\0', '/', '\\', '<', '>', ':', '"', '|', '?', '*'];

    // Trim spaces and check for reserved names
    let cleaned = filename.trim().to_string();

    // If the filename is empty or a reserved name, return `None`
    if cleaned.is_empty() || reserved_names.iter().any(|&reserved| reserved.eq_ignore_ascii_case(&cleaned)) {
        return None;
    }

    // Remove invalid characters and truncate to 255 characters
    let sanitized: String = cleaned
        .chars()
        .filter(|&c| !invalid_chars.contains(&c)) // Exclude invalid characters
        .collect();

    // Ensure the cleaned filename length does not exceed 255 characters
    Some(sanitized.chars().take(255).collect())
}
