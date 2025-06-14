use serde::Deserialize;

use crate::model::info::{clip::TrackClip, video::Video};

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackSupplement {
    pub id: String,
    #[serde(default)]
    #[serde(deserialize_with = "crate::model::utils::deserialize_maybe_vec")]
    pub lyrics: Vec<TrackSupplementLyrics>,
    #[serde(default)]
    #[serde(deserialize_with = "crate::model::utils::deserialize_maybe_vec")]
    pub videos: Vec<Video>,
    #[serde(default)]
    #[serde(deserialize_with = "crate::model::utils::deserialize_maybe_vec")]
    pub clips: Vec<TrackClip>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackSupplementLyrics {
    pub id: i32,
    pub lyrics: String,
    pub full_lyrics: String,
    pub has_rights: bool,
    pub text_language: String,
    pub show_translation: bool,
}
