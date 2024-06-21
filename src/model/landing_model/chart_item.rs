use serde::Deserialize;

use crate::model::track_model::track::{Track, TrackId};

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ChartItem {
    pub track: Option<Track>,
    pub chart: Option<Chart>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub position: i32,
    pub listeners: i32,
    pub shift: i32,
    pub bg_color: Option<String>,
    pub track_id: Option<TrackId>,
}
