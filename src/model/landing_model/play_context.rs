use serde::Deserialize;

use crate::model::track_model::track::TrackId;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayContext {
    pub client: String,
    pub context: String,
    pub context_item: String,
    pub tracks: Vec<IncompleteTrack>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncompleteTrack {
    pub track_id: TrackId,
    pub timestamp: String,
}
