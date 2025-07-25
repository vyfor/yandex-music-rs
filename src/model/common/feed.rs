use chrono::NaiveDate;
use serde::Deserialize;

use crate::model::{
    album::event::AlbumEvent,
    artist::event::ArtistEvent,
    landing::personal_playlist::PersonalPlaylist,
    track::{Track, TrackWithAds},
};

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    pub can_get_more_events: bool,
    pub pumpkin: bool,
    pub is_wizard_passed: bool,
    pub generated_playlists: Vec<PersonalPlaylist>,
    pub headlines: Vec<Headline>,
    pub today: NaiveDate,
    pub days: Vec<Day>,
    pub next_revision: Option<NaiveDate>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headline {
    #[serde(rename = "type")]
    pub item_type: String,
    pub id: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Day {
    pub day: NaiveDate,
    pub events: Vec<DayEvent>,
    pub tracks_to_play_with_ads: Vec<TrackWithAds>,
    pub tracks_to_play: Vec<Track>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayEvent {
    pub id: String,
    #[serde(rename = "type")]
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
    pub tracks_count: Option<u32>,
    pub genre: Option<String>,
}
