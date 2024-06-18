use serde::Deserialize;

use super::track::PartialTrack;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    pub uid: i32,
    pub revision: i32,
    pub tracks: Vec<PartialTrack>,
}
