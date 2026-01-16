use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::model::collection::CollectionInfo;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase", from = "LikedArtistsRaw")]
pub struct LikedArtistsCollection {
    pub info: CollectionInfo,
    pub liked: Vec<LikedArtist>,
    pub disliked: Vec<LikedArtist>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikedArtist {
    pub artist_id: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize)]
struct LikedArtistsRaw {
    info: CollectionInfo,
    values: LikedValues,
}

#[derive(Deserialize)]
struct LikedValues {
    liked: Vec<LikedArtist>,
    disliked: Vec<LikedArtist>,
}

impl From<LikedArtistsRaw> for LikedArtistsCollection {
    fn from(raw: LikedArtistsRaw) -> Self {
        Self {
            info: raw.info,
            liked: raw.values.liked,
            disliked: raw.values.disliked,
        }
    }
}
