pub mod context;
pub mod queue_item;
pub mod status;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::model::track::TrackId;
use context::QueueContext;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Queue {
    pub context: Option<QueueContext>,
    pub tracks: Vec<TrackId>,
    pub current_index: u32,
    pub modified: DateTime<Utc>,
    pub id: Option<String>,
    pub from: Option<String>,
}
