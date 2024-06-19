use serde::Deserialize;

use crate::model::track::Track;

use super::{
    icon::StationIcon, id::StationId, restrictions::StationRestrictions,
};

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub id: StationId,
    pub name: String,
    pub icon: StationIcon,
    pub mts_icon: StationIcon,
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
    pub r#type: String,
    pub track: Track,
    pub liked: bool,
}
