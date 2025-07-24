use serde::Deserialize;
use crate::UserId;

use crate::model::track::PartialTrack;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    pub uid: UserId,
    pub revision: i32,
    pub tracks: Vec<PartialTrack>,
}
