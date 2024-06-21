use crate::{
    api::{RequestPath, Response},
    model::search_model::suggestion::SearchSuggestion,
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
    pub async fn search_suggestion(
        &self,
        text: String,
    ) -> Result<SearchSuggestion, crate::ClientError> {
        let response: Response =
            self.get(&SearchSuggestionRequest::new(text).path()).await?;

        Ok(serde_json::from_value::<SearchSuggestion>(response.result)?)
    }
}
