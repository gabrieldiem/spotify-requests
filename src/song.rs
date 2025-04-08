use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Song {
    pub id: String,
    pub name: String,
}
