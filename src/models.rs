use serde::{Deserialize, Serialize};

pub enum OrganizerMode {
    Movies,
    TvShows,
}

impl From<String> for OrganizerMode {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "movies" => OrganizerMode::Movies,
            "tvshows" => OrganizerMode::TvShows,
            _ => panic!("Unknown organizer mode: {}", value),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieMetadata {
    pub(crate) original_file_name: String,
    pub(crate) media_name: String,
    pub(crate) release_year: Option<u16>,
    pub(crate) encoding_format: Option<MediaEncodingFormat>,
    pub(crate) resolution: Option<u16>,
    pub(crate) additional_data: Vec<String>,
    pub(crate) file_extension: String,
    pub(crate) imdb_id: Option<String>,
}
