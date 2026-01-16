use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::model::collection::CollectionInfo;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase", from = "LikedAlbumsRaw")]
pub struct LikedAlbumsCollection {
    pub info: CollectionInfo,
    pub liked: Vec<LikedAlbum>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikedAlbum {
    pub album_id: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize)]
struct LikedAlbumsRaw {
    info: CollectionInfo,
    values: LikedValues,
}

#[derive(Deserialize)]
struct LikedValues {
    liked: Vec<LikedAlbum>,
}

impl From<LikedAlbumsRaw> for LikedAlbumsCollection {
    fn from(raw: LikedAlbumsRaw) -> Self {
        Self {
            info: raw.info,
            liked: raw.values.liked,
        }
    }
}
