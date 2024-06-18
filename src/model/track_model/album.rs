use serde::Deserialize;

use super::{
    artist::Artist, custom_wave::CustomWave, label::Label, track::Track,
};

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: i32,
    pub error: Option<String>,
    pub title: Option<String>,
    pub track_count: Option<i32>,
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
    pub bests: Vec<i32>,
    #[serde(default)]
    pub duplicates: Vec<Album>,
    #[serde(default)]
    pub volumes: Vec<Vec<Track>>,
    pub year: Option<i32>,
    pub release_date: Option<String>,
    pub r#type: Option<String>,
    pub track_position: Option<TrackPosition>,
    #[serde(default)]
    pub regions: Vec<String>,
    pub available_as_rbt: Option<bool>,
    pub lyrics_available: Option<bool>,
    pub remember_position: Option<bool>,
    #[serde(default)]
    pub albums: Vec<Album>,
    pub duration_ms: Option<i32>,
    pub explicit: Option<bool>,
    pub start_date: Option<String>,
    pub likes_count: Option<i32>,
    #[serde(default)]
    pub available_regions: Vec<String>,
    #[serde(default)]
    pub available_for_options: Vec<String>,
    pub meta_tag_id: Option<String>,
    pub has_trailer: Option<bool>,
    pub sort_order: Option<String>,
    pub background_image_url: Option<String>,
    pub custom_wave: Option<CustomWave>,
    pub pager: Option<AlbumPager>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct TrackPosition {
    pub volume: i32,
    pub index: i32,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumPager {
    pub page: i32,
    pub per_page: i32,
    pub total: i32,
}
