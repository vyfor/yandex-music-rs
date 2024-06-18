use serde::Deserialize;

use super::track::Track;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimilarTracks {
    pub track: Option<Track>,
    #[serde(default)]
    pub similar_tracks: Vec<Track>,
}
