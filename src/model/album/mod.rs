pub mod event;

use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::model::{
    artist::Artist,
    info::pager::Pager,
    track::{custom_wave::CustomWave, label::Label, Track},
};

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    #[serde(default)]
    #[serde(deserialize_with = "crate::model::utils::opt_string_to_u32")]
    pub id: Option<u32>,
    pub error: Option<String>,
    pub title: Option<String>,
    pub track_count: Option<u32>,
    #[serde(default)]
    pub artists: Vec<Artist>,
    #[serde(default)]
    pub labels: Vec<Label>,
    pub available: Option<bool>,
    pub available_for_premium_users: Option<bool>,
    pub version: Option<String>,
    pub cover_uri: Option<String>,
    pub content_warning: Option<String>,
    pub genre: Option<String>,
    pub text_color: Option<String>,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub is_premiere: Option<bool>,
    pub is_banner: Option<bool>,
    pub meta_type: Option<String>,
    pub storage_dir: Option<String>,
    pub og_image: Option<String>,
    pub recent: Option<bool>,
    pub very_important: Option<bool>,
    pub available_for_mobile: Option<bool>,
    pub available_partially: Option<bool>,
    #[serde(default)]
    pub bests: Vec<u32>,
    #[serde(default)]
    pub duplicates: Vec<Album>,
    #[serde(default)]
    pub volumes: Vec<Vec<Track>>,
    pub year: Option<u16>,
    pub release_date: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    pub track_position: Option<TrackPosition>,
    #[serde(default)]
    pub regions: Vec<String>,
    pub available_as_rbt: Option<bool>,
    pub lyrics_available: Option<bool>,
    pub remember_position: Option<bool>,
    #[serde(default)]
    pub albums: Vec<Album>,
    #[serde(
        default,
        deserialize_with = "crate::model::utils::opt_duration_from_millis"
    )]
    pub duration_ms: Option<Duration>,
    pub explicit: Option<bool>,
    pub start_date: Option<DateTime<Utc>>,
    pub likes_count: Option<u32>,
    #[serde(default)]
    pub available_regions: Vec<String>,
    #[serde(default)]
    pub available_for_options: Vec<String>,
    pub meta_tag_id: Option<String>,
    pub has_trailer: Option<bool>,
    pub sort_order: Option<String>,
    pub background_image_url: Option<String>,
    pub custom_wave: Option<CustomWave>,
    pub pager: Option<Pager>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct TrackPosition {
    pub volume: u8,
    pub index: u32,
}
