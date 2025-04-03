const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

pub struct AuthData {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
}

pub struct AuthClient {
    req_client: reqwest::Client,
    client_id: String,
    client_secret: String,
}

impl AuthClient {
    pub fn new(client_id: &str, client_secret: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let req_client = reqwest::Client::builder().build()?;

        Ok(AuthClient {
            req_client,
            client_id: String::from(client_id),
            client_secret: String::from(client_secret),
        })
    }

    pub async fn authenticate(&self) -> Result<AuthData, Box<dyn std::error::Error>> {
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
        ];

        let res = self
            .req_client
            .post(TOKEN_URL)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .form(&params)
            .send()
            .await?
            .text()
            .await?;

        let response_data: serde_json::Value = serde_json::to_value(res.to_string()).unwrap();
        println!("{response_data:?}");

        let access_token = response_data["access_token"].to_string();
        let token_type = response_data["token_type"].to_string();
        let expires_in = response_data["expires_in"].to_string().parse::<u32>()?;

        Ok(AuthData {
            access_token,
            token_type,
            expires_in,
        })
    }
}
