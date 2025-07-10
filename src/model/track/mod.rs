pub mod cover;
pub mod custom_wave;
pub mod label;
pub mod similar_tracks;
pub mod supplement;

use serde::{Deserialize, Serialize};

use crate::model::{album::Album, artist::Artist, user::User};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialTrack {
    #[serde(deserialize_with = "crate::model::utils::number_to_string")]
    pub id: String,
    #[serde(default)]
    #[serde(deserialize_with = "crate::model::utils::opt_string_to_i32")]
    pub album_id: Option<i32>,
    pub timestamp: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackWithInfo {
    #[serde(deserialize_with = "crate::model::utils::opt_number_to_string")]
    pub id: Option<String>,
    pub original_index: i32,
    pub timestamp: String,
    pub track: Track,
    pub recent: bool,
    pub original_shuffle_index: i32,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    #[serde(deserialize_with = "crate::model::utils::number_to_string")]
    pub id: String,
    pub title: Option<String>,
    pub available: Option<bool>,
    #[serde(default)]
    pub artists: Vec<Artist>,
    #[serde(default)]
    pub albums: Vec<Album>,
    pub available_for_premium_users: Option<bool>,
    pub lyrics_available: Option<bool>,
    pub best: Option<bool>,
    #[serde(deserialize_with = "crate::model::utils::number_to_string")]
    pub real_id: String,
    pub og_image: Option<String>,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    pub cover_uri: Option<String>,
    pub major: Option<TrackMajor>,
    pub duration_ms: Option<i32>,
    pub storage_dir: Option<String>,
    pub file_size: Option<i32>,
    pub substituted: Option<Box<Track>>,
    pub matched_track: Option<Box<Track>>,
    #[serde(default)]
    pub normalization: Vec<TrackNormalization>,
    pub error: Option<String>,
    pub can_publish: Option<bool>,
    pub state: Option<String>,
    pub desired_visibility: Option<String>,
    pub filename: Option<String>,
    pub user_info: Option<User>,
    pub meta_data: Option<TrackMetadata>,
    #[serde(default)]
    pub regions: Vec<String>,
    pub available_as_rbt: Option<bool>,
    pub content_warning: Option<String>,
    pub explicit: Option<bool>,
    pub preview_duration_ms: Option<i32>,
    pub available_full_without_permission: Option<bool>,
    pub version: Option<String>,
    pub remember_position: Option<bool>,
    pub background_video_uri: Option<String>,
    pub short_description: Option<String>,
    pub is_suitable_for_children: Option<bool>,
    pub track_source: Option<String>,
    #[serde(default)]
    pub available_for_options: Vec<String>,
    pub r128: Option<TrackR128>,
    pub lyrics_info: Option<TrackLyricsInfo>,
    pub track_sharing_flag: Option<String>,
    #[serde(default)]
    pub disclaimers: Vec<String>,
    #[serde(default)]
    pub derived_colors: Option<TrackDerivedColors>,
    pub fade: Option<TrackFade>,
    #[serde(default)]
    pub special_audio_resources: Vec<String>,
    pub player_id: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackId {
    #[serde(deserialize_with = "crate::model::utils::opt_number_to_string")]
    pub id: Option<String>,
    #[serde(deserialize_with = "crate::model::utils::opt_number_to_string")]
    pub track_id: Option<String>,
    #[serde(deserialize_with = "crate::model::utils::opt_string_to_i32")]
    pub album_id: Option<i32>,
    pub from: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackWithAds {
    #[serde(rename = "type")]
    pub item_type: String,
    pub track: Track,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackShort {
    #[serde(deserialize_with = "crate::model::utils::number_to_string")]
    pub id: String,
    #[serde(default)]
    #[serde(deserialize_with = "crate::model::utils::opt_number_to_string")]
    pub album_id: Option<String>,
}

impl TrackShort {
    pub fn new(id: impl Into<String>, album_id: Option<String>) -> Self {
        Self {
            id: id.into(),
            album_id,
        }
    }

    pub fn from_id(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            album_id: None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackMajor {
    #[serde(deserialize_with = "crate::model::utils::number_to_string")]
    pub id: String,
    pub name: String,
    pub pretty_name: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct TrackR128 {
    pub i: f32,
    pub tp: f32,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackFade {
    pub in_start: f32,
    pub in_stop: f32,
    pub out_start: f32,
    pub out_stop: f32,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackDerivedColors {
    pub average: String,
    pub wave_text: String,
    pub mini_player: String,
    pub accent: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackLyricsInfo {
    pub has_available_sync_lyrics: bool,
    pub has_available_text_lyrics: bool,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct TrackNormalization {
    pub gain: f32,
    pub peak: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackMetadata {
    pub album: Option<String>,
    pub volume: Option<i32>,
    pub year: Option<i32>,
    pub number: Option<i32>,
    pub genre: Option<String>,
    pub lyricist: Option<String>,
    pub version: Option<String>,
    pub composer: Option<String>,
}
