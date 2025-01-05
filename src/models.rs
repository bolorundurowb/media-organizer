use std::fs::Metadata;

#[derive(Debug)]
pub enum MediaEncodingFormat {
    X264,
    X265,
    H264,
    H265,
}

impl MediaEncodingFormat {
    pub fn from(input: &str) -> Option<MediaEncodingFormat> {
        match input.to_lowercase().as_str() {
            "x264" | "x.264" => Some(MediaEncodingFormat::X264),
            "x265" | "x.265" => Some(MediaEncodingFormat::X265),
            "h264" | "h.264" => Some(MediaEncodingFormat::H264),
            "h265" | "h.265" => Some(MediaEncodingFormat::H265),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct MovieMetadata {
    pub(crate) original_file_name: String,
    pub(crate)media_name: String,
    pub(crate)release_year: Option<u8>,
    pub(crate)encoding_format: Option<MediaEncodingFormat>,
    pub(crate)resolution: Option<u8>,
    pub(crate)additional_data: Vec<String>,
    pub(crate)file_extension: String,
}

pub struct ParsedMovieFileData {
    file_path: String,
    metadata: Metadata,
}
