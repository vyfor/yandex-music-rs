use std::time::Duration;

use serde::Deserialize;

use crate::model::artist::Artist;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clip {
    pub clip_id: u32,
    pub title: String,
    pub player_id: Option<String>,
    pub uuid: Option<String>,
    pub thumbnail: Option<String>,
    pub preview_url: Option<String>,
    #[serde(deserialize_with = "crate::model::utils::duration_from_millis")]
    pub duration: Duration,
    #[serde(default)]
    pub track_ids: Vec<u32>,
    #[serde(default)]
    pub artists: Vec<Artist>,
    #[serde(default)]
    pub disclaimers: Vec<String>,
    pub explicit: Option<bool>,
}
