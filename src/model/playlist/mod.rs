pub mod id;
pub mod library;
pub mod modify;

use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::model::{
    info::tag::Tag,
    track::{cover::Cover, PartialTrack, Track, TrackWithInfo},
    user::User,
};

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub playlist_uuid: String,
    pub description: Option<String>,
    pub description_formatted: Option<String>,
    pub available: bool,
    pub collective: bool,
    pub cover: Cover,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    #[serde(
        rename = "durationMs",
        deserialize_with = "crate::model::utils::duration_from_millis"
    )]
    pub duration: Duration,
    pub is_banner: bool,
    pub is_premiere: bool,
    pub kind: u32,
    pub og_image: String,
    pub tracks: Option<PlaylistTracks>,
    pub owner: User,
    pub revision: u32,
    pub snapshot: u32,
    pub tags: Vec<Tag>,
    pub title: String,
    pub track_count: u32,
    pub uid: u64,
    pub visibility: String,
    #[serde(default)]
    pub likes_count: u32,
    #[serde(default)]
    pub similar_playlists: Vec<Playlist>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum PlaylistTracks {
    Full(Vec<Track>),
    WithInfo(Vec<TrackWithInfo>),
    Partial(Vec<PartialTrack>),
}
