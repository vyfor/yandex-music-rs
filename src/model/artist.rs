use serde::Deserialize;

use super::{cover::Cover, track::TrackFull};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub composer: bool,
    pub cover: Cover,
    pub id: i32,
    pub name: String,
    pub various: bool,
    #[serde(default)]
    pub popular_tracks: Vec<TrackFull>,
    #[serde(default)]
    pub tickets_available: bool,
    #[serde(default)]
    pub regions: Vec<String>,
    #[serde(default)]
    pub disclaimers: Vec<String>,
    #[serde(default)]
    pub genres: Vec<String>,
}
