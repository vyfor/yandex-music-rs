use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StationFeedback {
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub from: Option<String>,
    pub track_id: Option<String>,
    #[serde(
        rename = "totalPlayedSeconds",
        serialize_with = "crate::model::utils::opt_duration_to_secs_f64"
    )]
    pub total_played: Option<Duration>,
}
