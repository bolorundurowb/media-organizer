use crate::models::{MediaEncodingFormat, MovieMetadata};
use regex::Regex;

pub fn parse_to_movie_metadata(file_name: &str) -> MovieMetadata {
    let (raw_file_name, file_extension) = get_raw_file_name_and_extension(file_name);
    compose_movie_metadata(raw_file_name, file_extension)
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
    }
}

fn to_title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn get_raw_file_name_and_extension(file_name: &str) -> (&str, String) {
    if let Some(pos) = file_name.rfind('.') {
        let name = &file_name[..pos];
        let extension = file_name[pos + 1..].to_string();
        (name, extension)
    } else {
        (file_name, String::new())
    }
}
