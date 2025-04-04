mod auth;
mod market;
mod spotify;

use crate::spotify::Album;
use auth::AuthClient;
use market::Market;
use spotify::SpotifyClient;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::exit;

const ERROR_EXIT_CODE: i32 = -1;
const OK_EXIT_CODE: i32 = -1;

struct Color;
#[allow(dead_code)]
impl Color {
    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const WHITE: &'static str = "\x1b[37m";

    pub const BOLD_RED: &'static str = "\x1b[1;31m";
    pub const BOLD_GREEN: &'static str = "\x1b[1;32m";
    pub const BOLD_YELLOW: &'static str = "\x1b[1;33m";
    pub const BOLD_BLUE: &'static str = "\x1b[1;34m";
    pub const BOLD_WHITE: &'static str = "\x1b[1;37m";

    pub const RESET: &'static str = "\x1b[0m";
}

fn print_albums(albums: &Vec<Album>) {
    for album in albums {
        println!(
            "{}Album: {}{}",
            Color::BOLD_YELLOW,
            Color::RESET,
            album.name
        );

        if album.songs.is_empty() {
            continue;
        }

        println!("{}Songs:{}", Color::BOLD_GREEN, Color::RESET);
        for song in &album.songs {
            println!("     {}-{} {}", Color::BOLD_GREEN, Color::RESET, song.name);
        }
        println!();
    }
}

async fn async_main(
    client_id: String,
    client_secret: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let auth_client = AuthClient::new(&client_id, &client_secret)?;
    let mut spotify_client = SpotifyClient::new(auth_client)?;
    let albums_with_songs = spotify_client.get_new_albums(Market::argentina()).await?;
    print_albums(&albums_with_songs);
    Ok(())
}

fn load_env(filename: &str) -> Result<HashMap<String, String>, io::Error> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut client_credentials = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            client_credentials.insert(key.trim().to_owned(), value.trim().to_owned());
        }
    }

    Ok(client_credentials)
}

#[tokio::main]
async fn main() {
    match load_env(".env") {
        Ok(client_credentials) => {
            let client_id = match client_credentials.get("CLIENT_ID") {
                Some(id) => id.clone(),
                None => {
                    eprintln!("Error: CLIENT_ID is missing from .env");
                    exit(ERROR_EXIT_CODE);
                }
            };

            let client_secret = match client_credentials.get("CLIENT_SECRET") {
                Some(secret) => secret.clone(),
                None => {
                    eprintln!("Error: CLIENT_SECRET is missing from .env");
                    exit(ERROR_EXIT_CODE);
                }
            };

            let res = async_main(client_id, client_secret).await;
            match res {
                Ok(_) => {
                    exit(OK_EXIT_CODE);
                }
                Err(err) => {
                    eprintln!("Error in async_main: {}", err);
                    exit(ERROR_EXIT_CODE);
                }
            }
        }

        Err(err) => {
            eprintln!("Failed to load .env file: {}", err);
            exit(ERROR_EXIT_CODE);
        }
    }
}
