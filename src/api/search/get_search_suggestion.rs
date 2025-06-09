use crate::{
    api::Endpoint, client::request::RequestOptions, model::search::suggestion::SearchSuggestion,
    YandexMusicClient,
};
use reqwest::Method;
use std::borrow::Cow;

pub struct GetSearchSuggestionOptions {
    pub part: String,
}

impl GetSearchSuggestionOptions {
    pub fn new(part: impl Into<String>) -> Self {
        Self { part: part.into() }
    }
}

impl Endpoint for GetSearchSuggestionOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("search/suggest?part={}", self.part).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Get search suggestion.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the search text.
    ///
    /// ### Returns
    /// * `Result<SearchSuggestion, ClientError>` - The search suggestion or an error if the request fails.
    pub async fn get_search_suggestion(
        &self,
        options: &GetSearchSuggestionOptions,
    ) -> Result<SearchSuggestion, crate::ClientError> {
        self.request::<SearchSuggestion, _>(options).await
    }
}
