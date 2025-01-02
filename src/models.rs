use std::fs::Metadata;
use std::path::Path;

enum MediaEncodingFormat {
    X264,
    X265,
    H264,
    H265,
}

struct MediaMetadata {
    original_file_name: String,
    media_name: String,
    release_year: Option<u8>,
    encoding_format: Option<MediaEncodingFormat>,
    resolution: Option<u8>,
    additional_data: Vec<String>,
    file_extension: String,
}

pub struct ParsedFileData {
    file_path: Path,
    metadata: Metadata,
}
