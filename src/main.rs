mod auth;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::exit;

const ERROR_EXIT_CODE: i32 = -1;
const OK_EXIT_CODE: i32 = -1;

/*async fn example() -> Option<String> {
    let client = reqwest::Client::builder()
        .build()
        .ok()?;

    let body = client.get("https://api.spotify.com/v1/browse/new-releases")
        .send()
        .await.ok()?
        .text()
        .await.ok()?;

    Some(body)
}*/

async fn async_main(
    client_id: &str,
    client_secret: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let auth_client = auth::AuthClient::new(client_id, client_secret)?;
    let auth = auth_client.authenticate().await?;
    println!("{auth:?}");
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

            let res = async_main(&client_id, &client_secret).await;
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
