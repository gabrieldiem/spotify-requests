#[derive(Debug)]
#[allow(dead_code)]
pub struct AuthData {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
}
