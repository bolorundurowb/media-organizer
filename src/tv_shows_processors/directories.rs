use std::fs::{DirEntry, ReadDir};
use std::path::Path;
use inline_colorization::{color_green, color_reset};

pub async fn process_directories(directory_entries: Vec<DirEntry>)  {
    for directory in directory_entries {
        println!("{color_green}Processing directory {:?}{color_reset}", directory.path());
        process_directory(directory).await;
    }
}

async fn process_directory(directory_entry: DirEntry) {

}