use serde::Deserialize;

use crate::model::track::Track;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ChartItem {
    pub track: Option<Track>,
    pub chart: Option<PartialChart>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialChart {
    pub position: i32,
    pub listeners: i32,
    pub shift: i32,
    pub bg_color: Option<String>,
    pub track_id: Option<TrackId>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackId {
    pub id: i32,
    pub track_id: i32,
    pub album_id: Option<i32>,
    pub from: Option<String>,
}
