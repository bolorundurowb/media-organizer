mod constants;
mod imdb;
mod models;
mod processors;
mod subtitles;
mod utils;

use std::env::args;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use inline_colorization::{color_blue, color_green, color_reset};

#[tokio::main]
async fn main() {
    let directory_opt = args().nth(1);

    if directory_opt.is_none() {
        println!("{color_blue}Usage: media-organizer{color_reset} {color_green}[directory]{color_reset}");
        return;
    }

    let directory_path = Path::new(&directory_opt.unwrap());
    if !directory_path.exists() {
        panic!("Specified source path does not exist");
    }

    let files: Vec<DirEntry> = fs::read_dir(&directory_path)
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().is_file())
        .collect();

    // collect this into directories
    if files.len() > 0 {
        processors::files::process_files(&directory_path, files);
    }

    let directories = fs::read_dir(&directory_path)
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().is_dir())
        .collect();
    processors::directories::process_directories(directories).await;
}
