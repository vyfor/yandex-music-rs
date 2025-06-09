use serde::Serialize;

use crate::model::track::TrackShort;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct ModifyPlaylist {
    pub diff: ModifyPlaylistDiff,
    pub revision: i32,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct ModifyPlaylistDiff {
    pub op: String,
    pub at: i32,
    pub tracks: Vec<TrackShort>,
}
