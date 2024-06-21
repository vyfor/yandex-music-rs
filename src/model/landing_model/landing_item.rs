use std::fmt::Display;

use serde::Deserialize;

use crate::model::playlist::{Playlist, PlaylistId};

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LandingItem {
    pub id: Option<String>,
    pub item_type: String,
    pub type_for_from: String,
    pub title: String,
    pub menu: Option<LandingItemMenu>,
    pub chart: Option<Playlist>,
    #[serde(default)]
    pub new_releases: Vec<i32>,
    #[serde(default)]
    pub new_playlists: Vec<PlaylistId>,
    #[serde(default)]
    pub podcasts: Vec<i32>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LandingItemMenu {
    pub title: String,
    pub url: String,
    pub selected: Option<bool>,
}

pub enum LandingBlockType {
    NewPlaylists,
    NewReleases,
    Chart(Option<ChartType>),
    Podcasts,
}

pub enum ChartType {
    Russia,
    World,
}

impl Display for LandingBlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LandingBlockType::NewPlaylists => write!(f, "new-playlists"),
            LandingBlockType::NewReleases => write!(f, "new-releases"),
            LandingBlockType::Chart(_) => write!(f, "chart"),
            LandingBlockType::Podcasts => write!(f, "podcasts"),
        }
    }
}

impl Display for ChartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChartType::Russia => write!(f, "russia"),
            ChartType::World => write!(f, "world"),
        }
    }
}
