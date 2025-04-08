use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthData {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
}
