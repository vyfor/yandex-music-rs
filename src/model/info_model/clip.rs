use serde::Deserialize;

use crate::model::track_model::artist::Artist;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackClip {
    pub clip_id: i32,
    pub title: String,
    pub player_id: Option<String>,
    pub uuid: Option<String>,
    pub thumbnail: Option<String>,
    pub preview_url: Option<String>,
    pub duration: i32,
    #[serde(default)]
    pub track_ids: Vec<i32>,
    #[serde(default)]
    pub artists: Vec<Artist>,
    #[serde(default)]
    pub disclaimers: Vec<String>,
    pub explicit: Option<bool>,
}
