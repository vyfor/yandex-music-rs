use serde::Deserialize;

use crate::model::{info_model::icon::Icon, track_model::track::Track};

use super::{id::StationId, restrictions::StationRestrictions};

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub id: StationId,
    pub name: String,
    pub icon: Icon,
    pub mts_icon: Icon,
    pub id_for_from: String,
    pub restrictions: StationRestrictions,
    pub restrictions2: StationRestrictions,
    pub full_image_url: Option<String>,
    pub mts_full_image_url: Option<String>,
    pub parent_id: Option<StationId>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationTracks {
    pub id: Option<StationId>,
    pub batch_id: String,
    pub pumpkin: bool,
    pub sequence: Vec<Sequence>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sequence {
    #[serde(rename = "type")]
    pub item_type: String,
    pub track: Track,
    pub liked: bool,
}
