use serde::Deserialize;

use crate::model::collection::{
    liked_albums::LikedAlbumsCollection, liked_artists::LikedArtistsCollection,
    liked_clips::LikedClipsCollection, liked_playlists::LikedPlaylistsCollection,
    liked_tracks::LikedTracksCollection,
};

pub mod liked_albums;
pub mod liked_artists;
pub mod liked_clips;
pub mod liked_playlists;
pub mod liked_tracks;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    liked_albums: Option<LikedAlbumsCollection>,
    liked_artists: Option<LikedArtistsCollection>,
    liked_clips: Option<LikedClipsCollection>,
    liked_playlists: Option<LikedPlaylistsCollection>,
    liked_tracks: Option<LikedTracksCollection>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionInfo {
    pub revision: u64,
    pub checksum: CollectionChecksum,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionChecksum {
    pub album_ids_mur_mur2: Option<String>,
    pub liked_artist_ids_mur_mur2: Option<String>,
    pub disliked_artist_ids_mur_mur2: Option<String>,
    pub liked_clip_ids_mur_mur2: Option<String>,
    pub disliked_clip_ids_mur_mur2: Option<String>,
    pub liked_track_ids_mur_mur2: Option<String>,
    pub disliked_track_ids_mur_mur2: Option<String>,
}
