use serde::Deserialize;

use super::{cover::Cover, track::Track};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub composer: bool,
    pub cover: Cover,
    pub id: i32,
    pub name: String,
    pub various: bool,
    pub popular_tracks: Vec<Track>,
    pub tickets_available: bool,
    pub regions: Vec<String>,
}
