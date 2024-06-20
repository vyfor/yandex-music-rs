use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueContext {
    pub r#type: String,
    pub id: Option<String>,
    pub description: Option<String>,
}
