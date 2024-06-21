use serde::Deserialize;

use crate::model::{
    album_model::album::Album, artist_model::artist::Artist,
    info_model::video::Video, playlist_model::playlist::Playlist,
    track_model::track::Track,
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
