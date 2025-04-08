use crate::song::Song;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Album {
    pub id: String,
    pub total_tracks: u32,
    pub available_markets: Vec<String>,
    pub name: String,
    pub songs: Vec<Song>,
}
