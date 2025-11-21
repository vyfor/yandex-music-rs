use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult<T> {
    #[serde(rename = "type", default)]
    pub item_type: Option<String>,
    pub total: u32,
    pub per_page: u32,
    pub order: u32,
    pub results: Vec<T>,
}
