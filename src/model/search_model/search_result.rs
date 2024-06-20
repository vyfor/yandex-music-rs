use serde::Deserialize;

use super::search_result_item::SearchResultItem;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub r#type: String,
    pub total: i32,
    pub per_page: i32,
    pub order: i32,
    pub results: Vec<SearchResultItem>,
}
