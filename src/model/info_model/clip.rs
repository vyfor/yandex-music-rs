use serde::Deserialize;

use crate::model::track_model::artist::Artist;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackClip {
    pub clip_id: i32,
    pub title: String,
    pub player_id: String,
    pub uuid: String,
    pub thumbnail: String,
    pub preview_url: String,
    pub duration: i32,
    pub track_ids: Vec<i32>,
    pub artists: Vec<Artist>,
    pub disclaimers: Vec<String>,
    pub explicit: bool,
}
