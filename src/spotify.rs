use crate::auth::{AuthClient, AuthData};

const NEW_ALBUM_RELEASES_URL: &str = "https://api.spotify.com/v1/browse/new-releases";

#[derive(Debug)]
#[allow(dead_code)]
pub struct Album {
    pub id: String,
    pub total_tracks: u32,
    pub available_markets: Vec<String>,
    pub name: String,
    pub songs: Vec<Song>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Song {
    id: String,
    name: String,
}

pub struct SpotifyClient {
    auth_client: AuthClient,
    req_client: reqwest::Client,
    auth_data: Option<AuthData>,
}

impl SpotifyClient {
    pub fn new(auth_client: AuthClient) -> Result<SpotifyClient, Box<dyn std::error::Error>> {
        let req_client = reqwest::Client::builder().build()?;
        Ok(SpotifyClient {
            auth_client,
            req_client,
            auth_data: None,
        })
    }

    async fn authenticate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.auth_data {
            Some(_) => Ok(()),
            None => {
                self.auth_data = Some(self.auth_client.authenticate().await?);
                Ok(())
            }
        }
    }

    fn parse_an_album(
        &self,
        album_data: &serde_json::Value,
    ) -> Result<Album, Box<dyn std::error::Error>> {
        let id: String = String::from(album_data["id"].as_str().ok_or("Missing id")?);
        let name: String = String::from(album_data["name"].as_str().ok_or("Missing name")?);
        let total_tracks = album_data["total_tracks"]
            .as_i64()
            .ok_or("Missing total_tracks")? as u32;

        let markets_serde: &Vec<serde_json::Value> = album_data["available_markets"]
            .as_array()
            .ok_or("Missing available_markets")?;
        let available_markets: Vec<String> = markets_serde
            .iter()
            .map(|elem| match elem.as_str().ok_or("Missing string") {
                Ok(elem_str) => String::from(elem_str),
                Err(_) => "".to_string(),
            })
            .collect();

        Ok(Album {
            id,
            available_markets,
            name,
            total_tracks,
            songs: Vec::new(),
        })
    }

    fn parse_albums(
        &self,
        data: serde_json::Value,
    ) -> Result<Vec<Album>, Box<dyn std::error::Error>> {
        let albums: &serde_json::Value = &data["albums"];
        let items: &Vec<serde_json::Value> =
            albums["items"].as_array().ok_or("Missing items array")?;
        let albums = items
            .iter()
            .map(|value| self.parse_an_album(value))
            .collect::<Result<Vec<Album>, _>>()?;

        Ok(albums)
    }

    pub async fn fill_albums_with_songs(
        &self,
        _albums: &mut [Album],
        _country_code: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    pub async fn get_new_albums(
        &mut self,
        country_code: String,
    ) -> Result<Vec<Album>, Box<dyn std::error::Error>> {
        self.authenticate().await?;
        let t: &str = self.auth_data.as_ref().unwrap().token_type.as_str();
        let a: &str = self.auth_data.as_ref().unwrap().access_token.as_str();
        let auth_string = format!("{} {}", t, a);

        let response_data: serde_json::Value = self
            .req_client
            .get(NEW_ALBUM_RELEASES_URL)
            .header(reqwest::header::AUTHORIZATION, auth_string)
            .send()
            .await?
            .json()
            .await?;
        let mut albums = self.parse_albums(response_data)?;
        self.fill_albums_with_songs(&mut albums, &country_code)
            .await?;
        Ok(albums)
    }
}
