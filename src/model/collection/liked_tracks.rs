use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::model::collection::CollectionInfo;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase", from = "LikedTracksRaw")]
pub struct LikedTracksCollection {
    pub info: CollectionInfo,
    pub liked: Vec<LikedTrack>,
    pub disliked: Vec<LikedTrack>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikedTrack {
    pub track_id: String,
    pub album_id: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize)]
struct LikedTracksRaw {
    info: CollectionInfo,
    values: LikedValues,
}

#[derive(Deserialize)]
struct LikedValues {
    liked: Vec<LikedTrack>,
    disliked: Vec<LikedTrack>,
}

impl From<LikedTracksRaw> for LikedTracksCollection {
    fn from(raw: LikedTracksRaw) -> Self {
        Self {
            info: raw.info,
            liked: raw.values.liked,
            disliked: raw.values.disliked,
        }
    }
}
