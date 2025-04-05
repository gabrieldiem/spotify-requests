use crate::auth::auth_data::AuthData;

const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

pub struct AuthClient {
    pub req_client: reqwest::Client,
    client_id: String,
    client_secret: String,
}

impl AuthClient {
    pub fn new(
        client_id: &str,
        client_secret: &str,
    ) -> Result<AuthClient, Box<dyn std::error::Error>> {
        let req_client = reqwest::Client::builder().build()?;

        Ok(AuthClient {
            req_client,
            client_id: String::from(client_id),
            client_secret: String::from(client_secret),
        })
    }

    async fn get_access_token(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
        ];

        let response = self
            .req_client
            .post(TOKEN_URL)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn authenticate(&self) -> Result<AuthData, Box<dyn std::error::Error>> {
        let response_data = self.get_access_token().await?;

        let access_token = String::from(
            response_data["access_token"]
                .as_str()
                .ok_or("Missing access_token")?,
        );
        let token_type = String::from(
            response_data["token_type"]
                .as_str()
                .ok_or("Missing token_type")?,
        );
        let expires_in = response_data["expires_in"]
            .as_i64()
            .ok_or("Missing expires_in")? as u32;

        Ok(AuthData {
            access_token,
            token_type,
            expires_in,
        })
    }
}
