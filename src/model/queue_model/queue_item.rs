use serde::Deserialize;

use super::context::QueueContext;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueItem {
    pub id: String,
    pub modified: String,
    pub context: Option<QueueContext>,
}
