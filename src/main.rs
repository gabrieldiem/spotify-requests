mod auth;
mod color;
mod market;
mod spotify;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::exit;

use auth::auth_client::AuthClient;
use color::Color;
use market::Market;
use spotify::Album;
use spotify::SpotifyClient;

const ERROR_EXIT_CODE: i32 = -1;
const OK_EXIT_CODE: i32 = -1;

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
    let auth_client = AuthClient::new(&client_id, &client_secret)?;
    let mut spotify_client = SpotifyClient::new(auth_client)?;
    let albums_with_songs = spotify_client.get_new_albums(Market::ARGENTINA).await?;
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

fn parse_env(
    client_credentials: &HashMap<String, String>,
    client_id: &mut String,
    client_secret: &mut String,
) {
    *client_id = match client_credentials.get("CLIENT_ID") {
        Some(id) => id.clone(),
        None => {
            eprintln!("Error: CLIENT_ID is missing from .env");
            exit(ERROR_EXIT_CODE);
        }
    };

    *client_secret = match client_credentials.get("CLIENT_SECRET") {
        Some(secret) => secret.clone(),
        None => {
            eprintln!("Error: CLIENT_SECRET is missing from .env");
            exit(ERROR_EXIT_CODE);
        }
    };
}

#[tokio::main]
async fn main() {
    match load_env(".env") {
        Ok(client_credentials) => {
            let mut client_id: String = String::new();
            let mut client_secret: String = String::new();

            parse_env(&client_credentials, &mut client_id, &mut client_secret);

            match fetch_and_print_albums(client_id, client_secret).await {
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
