use crate::{
    api::RequestPath, model::search_model::suggestion::SearchSuggestion,
    YandexMusicClient,
};

pub struct SearchSuggestionRequest {
    pub part: String,
}

impl SearchSuggestionRequest {
    pub fn new(part: String) -> Self {
        Self { part }
    }
}

impl RequestPath for SearchSuggestionRequest {
    fn path(&self) -> String {
        format!("search/suggest?part={}", self.part)
    }
}

impl YandexMusicClient {
    /// Get search suggestion.
    ///
    /// ### Arguments
    /// * `text` - The text to get the search suggestion.
    ///
    /// ### Returns
    /// * [SearchSuggestion] - The search suggestion for the text.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn search_suggestion(
        &self,
        text: String,
    ) -> Result<SearchSuggestion, crate::ClientError> {
        let response =
            self.get(&SearchSuggestionRequest::new(text).path()).await?;

        Ok(serde_json::from_value::<SearchSuggestion>(response)?)
    }
}
