mod constants;
mod imdb;
mod models;
mod movie_processors;
mod subtitles;
mod utils;
mod movies;
mod tv_shows_processors;
mod tv_shows;

use std::env::args;
use std::fs;
use std::path::Path;
use inline_colorization::{color_green, color_reset, color_cyan};
use crate::models::OrganizerMode;
use crate::movies::handle_movies;
use crate::tv_shows::handle_tv_shows;

#[tokio::main]
async fn main() {
    let command_opt = args().nth(1);

    if command_opt.is_none() {
        print_help();
        return;
    }

    let directory_opt = args().nth(2);

    if directory_opt.is_none() {
        print_help();
        return;
    }

    let directory = directory_opt.map(String::from).unwrap();
    let directory_path = Path::new(&directory);
    if !directory_path.exists() {
        panic!("Specified source path does not exist");
    }

    let command = command_opt.map(OrganizerMode::from).unwrap();
    let dir_entries = fs::read_dir(&directory_path).unwrap();

    println!();
    println!("Processing media directory: '{}'", &directory_path.to_string_lossy());
    println!();

    match command {
        OrganizerMode::Movies => {
            handle_movies(&directory_path, dir_entries).await;
        }
        OrganizerMode::TvShows => {
            handle_tv_shows(&directory_path, dir_entries).await;
        }
    }
}

fn print_help() {
    println!("Welcome to Media Organizer");
    println!();
    println!("{color_green}Usage:{color_reset} {color_cyan}media-organizer [command] [directory path]{color_reset}");
    println!();
    println!("{color_green}Commands:{color_reset}");
    println!("     {color_cyan}movies{color_reset}           Reorganize your movie directory");
    println!("     {color_cyan}tvshows{color_reset}          Reorganize your TV Series directory");
    println!(" {color_cyan}-h, --help{color_reset}           Print help");
    println!();
}
