use crate::album::Album;
use crate::auth::auth_client::AuthClient;
use crate::auth::auth_data::AuthData;
use crate::song::Song;

const NEW_ALBUM_RELEASES_URL: &str = "https://api.spotify.com/v1/browse/new-releases";

pub struct SpotifyClient<'a> {
    auth_client: AuthClient<'a>,
    http_client: &'a reqwest::Client,
    auth_data: Option<AuthData>,
}

impl SpotifyClient<'_> {
    pub fn new<'a>(
        auth_client: AuthClient<'a>,
        http_client: &'a reqwest::Client,
    ) -> SpotifyClient<'a> {
        SpotifyClient {
            auth_client,
            http_client,
            auth_data: None,
        }
    }

    fn songs_by_album_url(id: &str) -> String {
        format!("https://api.spotify.com/v1/albums/{id}/tracks")
    }

    async fn authenticate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.auth_data.is_none() {
            self.auth_data = Some(self.auth_client.authenticate().await?);
        }
        Ok(())
    }

    fn str_from_value(
        &self,
        data: &serde_json::Value,
        key: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(data[key]
            .as_str()
            .ok_or(format!("Missing {key}"))?
            .to_string())
    }

    fn parse_an_album(
        &self,
        album_data: &serde_json::Value,
    ) -> Result<Album, Box<dyn std::error::Error>> {
        let id = self.str_from_value(album_data, "id")?;
        let name: String = self.str_from_value(album_data, "name")?;
        let total_tracks = album_data["total_tracks"]
            .as_i64()
            .ok_or("Missing total_tracks")? as u32;

        let markets_serde: &Vec<serde_json::Value> = album_data["available_markets"]
            .as_array()
            .ok_or("Missing available_markets")?;

        let mut available_markets: Vec<String> = Vec::new();
        for market in markets_serde {
            match market.as_str().ok_or("Missing string") {
                Ok(elem_str) => available_markets.push(String::from(elem_str)),
                Err(_) => continue,
            }
        }

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
        let items = data["albums"]["items"]
            .as_array()
            .ok_or("Missing items array")?;
        let mut albums: Vec<Album> = Vec::new();

        for album in items {
            albums.push(self.parse_an_album(album)?);
        }

        Ok(albums)
    }

    fn parse_a_song(
        &self,
        song_data: &serde_json::Value,
    ) -> Result<Song, Box<dyn std::error::Error>> {
        let id = self.str_from_value(song_data, "id")?;
        let name = self.str_from_value(song_data, "name")?;

        Ok(Song { id, name })
    }

    async fn fill_albums_with_songs(
        &mut self,
        albums: &mut [Album],
        country_code: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let auth_string = self.get_auth_string().await?;

        for album in albums {
            let params = [
                ("id", album.id.clone()),
                ("market", country_code.to_string()),
            ];
            let mut url: String = SpotifyClient::songs_by_album_url(&album.id);
            url = reqwest::Url::parse_with_params(&url, &params)?.to_string();
            let response_data: serde_json::Value = self
                .http_client
                .get(url)
                .header(reqwest::header::AUTHORIZATION, auth_string.as_str())
                .send()
                .await?
                .json()
                .await?;

            let items = response_data["items"].as_array().ok_or("Missing items")?;

            for item in items {
                let song = self.parse_a_song(item)?;
                album.songs.push(song);
            }
        }
        Ok(())
    }

    async fn get_auth_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.authenticate().await?;
        let auth_data = self.auth_data.as_ref().unwrap();
        Ok(format!(
            "{} {}",
            auth_data.token_type, auth_data.access_token
        ))
    }

    pub async fn get_new_albums(
        &mut self,
        country_code: &str,
    ) -> Result<Vec<Album>, Box<dyn std::error::Error>> {
        let auth_string = self.get_auth_string().await?;

        let response_data: serde_json::Value = self
            .http_client
            .get(NEW_ALBUM_RELEASES_URL)
            .header(reqwest::header::AUTHORIZATION, auth_string)
            .send()
            .await?
            .json()
            .await?;

        let mut albums = self.parse_albums(response_data)?;
        self.fill_albums_with_songs(&mut albums, country_code)
            .await?;
        Ok(albums)
    }
}
