use serde::Deserialize;

use super::best_result::BestResult;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSuggestion {
    pub best: Option<BestResult>,
    pub suggestions: Vec<String>,
}
