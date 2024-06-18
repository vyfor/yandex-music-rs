use serde::Deserialize;

use super::{cover::Cover, track::Track};

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
    pub r#type: String,
    pub social_network: Option<String>,
}
