use serde::Deserialize;

use crate::model::info::{clip::TrackClip, video::Video};

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackSupplement {
    pub id: String,
    #[serde(default)]
    pub lyrics: Vec<TrackSupplementLyrics>,
    #[serde(default)]
    pub videos: Vec<Video>,
    #[serde(default)]
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
    pub show_translations: bool,
}
