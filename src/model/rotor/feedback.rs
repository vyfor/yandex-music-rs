use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StationFeedback {
    #[serde(rename = "type")]
    pub item_type: String,
    pub timestamp: DateTime<Utc>,
    pub from: String,
    pub track_id: String,
    #[serde(
        rename = "totalPlayedSeconds",
        serialize_with = "crate::model::utils::duration_to_secs"
    )]
    pub total_played: Duration,
}
