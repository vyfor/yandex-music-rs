use serde::Deserialize;

use super::play_context::IncompleteTrack;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayContexts {
    pub other_tracks: Vec<IncompleteTrack>,
}
