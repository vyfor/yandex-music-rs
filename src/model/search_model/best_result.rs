use serde::Deserialize;

use super::search_result_item::SearchResultItem;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BestResult {
    pub r#type: String,
    pub text: Option<String>,
    pub result: SearchResultItem,
}
