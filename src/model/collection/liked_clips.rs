use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::model::collection::CollectionInfo;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase", from = "LikedClipsRaw")]
pub struct LikedClipsCollection {
    pub info: CollectionInfo,
    pub liked: Vec<LikedClip>,
    pub disliked: Vec<LikedClip>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikedClip {
    pub clip_id: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize)]
struct LikedClipsRaw {
    info: CollectionInfo,
    values: LikedValues,
}

#[derive(Deserialize)]
struct LikedValues {
    liked: Vec<LikedClip>,
    disliked: Vec<LikedClip>,
}

impl From<LikedClipsRaw> for LikedClipsCollection {
    fn from(raw: LikedClipsRaw) -> Self {
        Self {
            info: raw.info,
            liked: raw.values.liked,
            disliked: raw.values.disliked,
        }
    }
}
