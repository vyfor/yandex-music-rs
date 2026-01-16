use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase", from = "LikedPlaylistsRaw")]
pub struct LikedPlaylistsCollection {
    pub liked: Vec<LikedPlaylist>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikedPlaylist {
    pub composite_data: LikedPlaylistData,
    pub info: LikedPlaylistInfo,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikedPlaylistData {
    pub uid: u64,
    pub kind: u32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikedPlaylistInfo {
    pub revision: u64,
    pub snapshot: u64,
}

#[derive(Deserialize)]
struct LikedPlaylistsRaw {
    values: LikedValues,
}

#[derive(Deserialize)]
struct LikedValues {
    liked: Vec<LikedPlaylist>,
}

impl From<LikedPlaylistsRaw> for LikedPlaylistsCollection {
    fn from(raw: LikedPlaylistsRaw) -> Self {
        Self {
            liked: raw.values.liked,
        }
    }
}
