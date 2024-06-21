use serde::Deserialize;

use crate::model::track_model::track::Track;

use super::album::Album;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumEvent {
    pub album: Option<Album>,
    pub tracks: Vec<Track>,
}
