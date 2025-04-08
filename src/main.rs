mod album;
mod auth;
mod color;
mod market;
mod song;
mod spotify;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::exit;

use album::Album;
use auth::auth_client::AuthClient;
use color::Color;
use market::Market;
use spotify::SpotifyClient;

const ERROR_EXIT_CODE: i32 = 1;
const OK_EXIT_CODE: i32 = 0;

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

async fn fetch_and_print_albums(
    client_id: String,
    client_secret: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let auth_client = AuthClient::new(client_id, client_secret)?;
    let mut spotify_client = SpotifyClient::new(auth_client)?;
    let albums_with_songs = spotify_client.get_new_albums(Market::ARGENTINA).await?;
    print_albums(&albums_with_songs);
    Ok(())
}

fn load_credentials(env_filename: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let env_file = File::open(env_filename)?;
    let reader = io::BufReader::new(env_file);
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

    let client_id = client_credentials
        .get("CLIENT_ID")
        .ok_or("CLIENT_ID is missing from .env")?
        .clone();

    let client_secret = client_credentials
        .get("CLIENT_SECRET")
        .ok_or("CLIENT_SECRET is missing from .env")?
        .clone();

    Ok((client_id, client_secret))
}

#[tokio::main]
async fn main() {
    // Use ? operator instead of match for simpler error handling
    let (client_id, client_secret) = match load_credentials(".env") {
        Ok(creds) => creds,
        Err(e) => {
            eprintln!("Failed to load credentials: {}", e);
            exit(ERROR_EXIT_CODE);
        }
    };

    if let Err(e) = fetch_and_print_albums(client_id, client_secret).await {
        eprintln!("Error in fetch_and_print_albums: {}", e);
        exit(ERROR_EXIT_CODE);
    }

    exit(OK_EXIT_CODE);
}
