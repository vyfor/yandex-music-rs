use serde::Deserialize;

use crate::model::track::PartialTrack;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    pub uid: u64,
    pub revision: u32,
    pub tracks: Vec<PartialTrack>,
}
