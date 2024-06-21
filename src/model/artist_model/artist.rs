use std::fmt::Display;

use serde::Deserialize;

use crate::model::{
    info_model::clip::TrackClip, info_model::pager::Pager,
    info_model::video::Video, landing_model::chart_item::Chart,
};

use crate::model::{
    album_model::album::Album, track_model::cover::Cover,
    track_model::custom_wave::CustomWave, track_model::track::Track,
};

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: i32,
    pub error: Option<String>,
    pub reason: Option<String>,
    pub name: Option<String>,
    pub cover: Option<Cover>,
    pub various: Option<bool>,
    pub composer: Option<bool>,
    pub genres: Option<Vec<String>>,
    pub og_image: Option<String>,
    pub op_image: Option<String>,
    pub counts: Option<ArtistCounts>,
    pub available: Option<bool>,
    pub ratings: Option<ArtistRatings>,
    #[serde(default)]
    pub links: Vec<ArtistLink>,
    pub tickets_available: Option<bool>,
    pub likes_count: Option<i32>,
    #[serde(default)]
    pub popular_tracks: Vec<Track>,
    #[serde(default)]
    pub regions: Vec<String>,
    #[serde(default)]
    pub decomposed: Vec<String>,
    pub description: Option<ArtistDescription>,
    #[serde(default)]
    pub countries: Vec<String>,
    pub en_wikipedia_link: Option<String>,
    #[serde(default)]
    pub db_aliases: Vec<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub init_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistInfo {
    pub artist: Artist,
    #[serde(default)]
    pub albums: Vec<Album>,
    #[serde(default)]
    pub also_albums: Vec<Album>,
    #[serde(default)]
    pub latest_release_ids: Vec<i32>,
    #[serde(default)]
    pub popular_tracks: Vec<Track>,
    pub bandlink_scanner_link: Option<BandlinkLink>,
    #[serde(default)]
    pub similar_artists: Vec<Artist>,
    #[serde(default)]
    pub all_covers: Vec<Cover>,
    #[serde(default)]
    pub videos: Vec<Video>,
    #[serde(default)]
    pub clips: Vec<TrackClip>,
    #[serde(default)]
    pub latest_releases: Vec<Album>,
    pub custom_waves: Option<CustomWave>,
    pub has_promotions: bool,
    #[serde(default)]
    pub tracks_in_chart: Vec<Chart>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistTrackIds {
    pub artist: Artist,
    pub tracks: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistTracks {
    pub pager: Pager,
    pub tracks: Vec<Track>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistAlbums {
    pub pager: Pager,
    pub albums: Vec<Album>,
}

pub enum SortBy {
    Year,
    Rating,
}

impl Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Year => write!(f, "year"),
            Self::Rating => write!(f, "rating"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BandlinkLink {
    pub title: String,
    pub subtitle: String,
    pub url: String,
    pub img_url: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistCounts {
    pub tracks: i32,
    pub direct_albums: i32,
    pub also_albums: i32,
    pub also_tracks: i32,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistRatings {
    pub tracks: i32,
    pub direct_albums: i32,
    pub also_albums: i32,
    pub also_tracks: i32,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistDescription {
    pub text: String,
    pub uri: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistLink {
    pub title: String,
    pub href: String,
    pub item_type: String,
    pub social_network: Option<String>,
}
