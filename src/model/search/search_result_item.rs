use serde::Deserialize;

use crate::model::{
    album::Album, artist::Artist, info::video::Video, playlist::Playlist, track::Track, user::User,
};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum SearchResultItem {
    Track(Track),
    Artist(Artist),
    Album(Album),
    Playlist(Playlist),
    Video(Video),
    User(User),
    #[serde(rename = "podcast")]
    Podcast(Album),
    #[serde(rename = "podcast_episode")]
    PodcastEpisode(Track),
    #[serde(rename = "single")]
    Single(Album),
}
