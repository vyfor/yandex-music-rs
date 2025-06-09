use serde::Deserialize;

use crate::model::track::Track;

use super::Artist;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistEvent {
    pub artist: Option<Artist>,
    pub tracks: Vec<Track>,
    #[serde(default)]
    pub similar_to_artists_from_history: Vec<Artist>,
    pub subscribed: Option<bool>,
}
