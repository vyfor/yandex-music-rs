use serde::Deserialize;

use super::search_result_item::SearchResultItem;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BestResult {
    #[serde(rename = "type")]
    pub item_type: String,
    pub text: Option<String>,
    pub result: SearchResultItem,
}
