use serde::Deserialize;

use crate::model::{
    album::Album, artist::Artist, info::video::Video, playlist::Playlist,
    track::Track,
};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum SearchResultItem {
    Track(Track),
    Artist(Artist),
    Album(Album),
    Playlist(Playlist),
    Video(Video),
}
