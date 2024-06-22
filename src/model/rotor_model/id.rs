use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationId {
    #[serde(rename="type")]
    pub item_type: String,
    pub tag: String,
}
