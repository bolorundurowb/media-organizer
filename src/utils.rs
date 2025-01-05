use crate::models::{MediaEncodingFormat, MovieMetadata};
use std::fs::DirEntry;
use std::path::Path;
use std::{fs, io};

pub fn parse_to_movie_metadata(file_name: &str) -> MovieMetadata {
    let (raw_file_name, file_extension) = get_raw_file_name_and_extension(file_name);
    let file_parts = split_file_name(raw_file_name);
    compose_movie_metadata(file_parts, file_extension)
}

pub fn get_dir_entry(path: &Path) -> io::Result<DirEntry> {
    // Check if the path exists and is a directory or a file.
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Path not found"));
    }

    // Attempt to read the directory entry. This works for both files and directories.
    fs::read_dir(path.parent().unwrap_or(Path::new(".")))?
        .find(|entry_result| match entry_result {
            Ok(entry) => entry.path() == path,
            Err(_) => false,
        })
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "Entry not found in parent directory",
            )
        })?
        .map_err(|e| e)
}

fn compose_movie_metadata(file_parts: Vec<String>, file_extension: String) -> MovieMetadata {
    let mut original_file_name = file_parts.join(" ");
    let mut media_name = String::new();
    let mut release_year = None;
    let mut encoding_format = None;
    let mut resolution = None;
    let mut additional_data = Vec::new();

    for part in &file_parts {
        if release_year.is_none() {
            if let Ok(year) = part.parse::<u16>() {
                if year >= 1900 && year <= 2100 {
                    release_year = Some(year);
                    continue;
                }
            }
        }

        if resolution.is_none() {
            if let Ok(res) = part.parse::<u16>() {
                if res == 720 || res == 1080 || res == 2160 || res == 4 {
                    resolution = Some(if res == 4 { 2160 } else { res });
                    continue;
                }
            }
        }

        if encoding_format.is_none() {
            if let Some(format) = MediaEncodingFormat::from(part) {
                encoding_format = Some(format);
                continue;
            }
        }

        additional_data.push(part.clone());
    }

    // Determine media_name by removing the year and other metadata.
    if let Some(year) = release_year {
        let year_str = year.to_string();
        if let Some(index) = original_file_name.find(&year_str) {
            media_name = original_file_name[..index].trim().to_string();
            original_file_name = original_file_name[index + year_str.len()..]
                .trim()
                .to_string();
        }
    }

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

fn split_file_name(file_name: &str) -> Vec<String> {
    if let Some(space_split) = split_file_name_by_delimiter(file_name, " ") {
        space_split
    } else if let Some(period_split) = split_file_name_by_delimiter(file_name, ".") {
        period_split
    } else if let Some(underscore_split) = split_file_name_by_delimiter(file_name, "_") {
        underscore_split
    } else {
        vec![file_name.to_string()]
    }
}

fn split_file_name_by_delimiter(file_name: &str, delimiter: &str) -> Option<Vec<String>> {
    if file_name.contains(delimiter) {
        Some(file_name.split(delimiter).map(String::from).collect())
    } else {
        None
    }
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
