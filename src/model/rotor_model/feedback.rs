use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StationFeedback {
    pub item_type: String,
    pub timestamp: String,
    pub from: String,
    pub track_id: String,
    pub total_played_seconds: i32,
}
