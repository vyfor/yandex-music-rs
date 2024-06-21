use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueStatus {
    pub status: String,
    pub most_recent_queue: Option<String>,
}
