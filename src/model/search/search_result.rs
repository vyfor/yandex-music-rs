use serde::Deserialize;

use super::search_result_item::SearchResultItem;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    #[serde(rename="type")]
    pub item_type: Option<String>,
    pub total: u32,
    pub per_page: u32,
    pub order: u32,
    pub results: Vec<SearchResultItem>,
}
