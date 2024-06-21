use serde::Deserialize;

use crate::model::{
    album::AlbumEvent,
    artist::ArtistEvent,
    personal_playlist::PersonalPlaylist,
    track::{Track, TrackWithAds},
};

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    pub can_get_more_events: bool,
    pub pumpkin: bool,
    pub is_wizard_passed: bool,
    pub generated_playlists: Vec<PersonalPlaylist>,
    pub headlines: Vec<String>,
    pub today: String,
    pub days: Vec<Day>,
    pub next_revision: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Day {
    pub day: String,
    pub events: Vec<DayEvent>,
    pub tracks_to_play_with_ads: Vec<TrackWithAds>,
    pub tracks_to_play: Vec<Track>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayEvent {
    pub id: String,
    pub item_type: String,
    pub type_for_from: Option<String>,
    pub title: Option<String>,
    #[serde(default)]
    pub tracks: Vec<Track>,
    #[serde(default)]
    pub artists: Vec<ArtistEvent>,
    #[serde(default)]
    pub albums: Vec<AlbumEvent>,
    pub message: Option<String>,
    pub device: Option<String>,
    pub tracks_count: Option<i32>,
    pub genre: Option<String>,
}
