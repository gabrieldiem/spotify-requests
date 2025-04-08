use crate::auth::auth_data::AuthData;

const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

pub struct AuthClient {
    pub req_client: reqwest::Client,
    client_id: String,
    client_secret: String,
}

impl AuthClient {
    pub fn new(
        client_id: String,
        client_secret: String,
    ) -> Result<AuthClient, Box<dyn std::error::Error>> {
        let req_client = reqwest::Client::builder().build()?;
        Ok(AuthClient {
            req_client,
            client_id,
            client_secret,
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
        Ok(serde_json::from_value(response_data)?)
    }
}
