use serde::Deserialize;

use crate::model::{
    album_model::album::Album, playlist_model::playlist::Playlist,
};

use super::{
    chart_item::ChartItem, mix_link::MixLink,
    personal_playlist::PersonalPlaylist, play_context::PlayContext,
    promotion::Promotion,
};

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Landing {
    pub pumpkin: bool,
    pub content_id: String,
    pub blocks: Vec<LandingBlock>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LandingBlock {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub type_for_from: String,
    pub title: Option<String>,
    pub entities: Vec<BlockEntity>,
    pub description: Option<String>,
    pub data: Option<BlockData>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum BlockData {
    PersonalPlaylistsData,
    PlayContextsData,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockEntity {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub data: EntityData,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum EntityData {
    PersonalPlaylist(PersonalPlaylist),
    Promotion(Promotion),
    Album(Album),
    Playlist(Playlist),
    ChartItem(ChartItem),
    PlayContext(PlayContext),
    MixLink(MixLink),
}

pub enum LandingType {
    PersonalPlaylists,
    Promotions,
    NewReleases,
    NewPlaylists,
    Mixes,
    Chart,
    Artists,
    Albums,
    Playlists,
    PlayContexts,
    Podcasts,
}

impl LandingType {
    pub fn as_str(&self) -> &str {
        match self {
            LandingType::PersonalPlaylists => "personalplaylists",
            LandingType::Promotions => "promotions",
            LandingType::NewReleases => "new-releases",
            LandingType::NewPlaylists => "new-playlists",
            LandingType::Mixes => "mixes",
            LandingType::Chart => "chart",
            LandingType::Artists => "artists",
            LandingType::Albums => "albums",
            LandingType::Playlists => "playlists",
            LandingType::PlayContexts => "play_contexts",
            LandingType::Podcasts => "podcasts",
        }
    }
}
