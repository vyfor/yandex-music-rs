use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::context::QueueContext;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueItem {
    pub id: String,
    pub modified: DateTime<Utc>,
    pub context: Option<QueueContext>,
}
