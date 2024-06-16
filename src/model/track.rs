use serde::Deserialize;

use super::{album::Album, artist::Artist};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: i32,
    pub play_count: i32,
    pub recent: bool,
    pub timestamp: String,
    pub track: TrackInfo,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackInfo {
    pub albums: Vec<Album>,
    pub artists: Vec<Artist>,
    pub available: bool,
    pub available_for_premium_users: bool,
    pub available_full_without_permission: bool,
    pub cover_uri: String,
    pub duration_ms: i32,
    pub file_size: i32,
    pub id: String,
    pub lyrics_available: bool,
    pub major: TrackMajor,
    pub normalization: TrackNormalization,
    pub og_image: String,
    pub preview_duration_ms: i32,
    pub real_id: String,
    pub remember_position: bool,
    pub storage_dir: String,
    pub title: String,
    pub r#type: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackMajor {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackNormalization {
    pub gain: i32,
    pub peak: i32,
}
