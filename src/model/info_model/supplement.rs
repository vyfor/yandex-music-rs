use serde::Deserialize;

use super::{clip::TrackClip, lyrics::TrackLyrics, video::TrackVideo};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackSupplement {
    pub id: i32,
    #[serde(default)]
    pub lyrics: Vec<TrackLyrics>,
    #[serde(default)]
    pub videos: Vec<TrackVideo>,
    #[serde(default)]
    pub clips: Vec<TrackClip>,
}
