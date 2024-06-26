use crate::{
    api::RequestPath,
    model::search_model::search::{Search, SearchType},
    YandexMusicClient,
};

pub struct SearchRequest {
    pub text: String,
    pub page: i32,
    pub item_type: SearchType,
    pub nocorrect: bool,
}

impl SearchRequest {
    pub fn new(text: String) -> Self {
        Self {
            text,
            page: 0,
            item_type: SearchType::All,
            nocorrect: false,
        }
    }

    pub fn with_page(mut self, page: i32) -> Self {
        self.page = page;
        self
    }

    pub fn with_type(mut self, item_type: SearchType) -> Self {
        self.item_type = item_type;
        self
    }

    pub fn with_nocorrect(mut self, nocorrect: bool) -> Self {
        self.nocorrect = nocorrect;
        self
    }
}

impl RequestPath for SearchRequest {
    fn path(&self) -> String {
        format!(
            "search/?text={}&page={}&type={}&nocorrect={}",
            self.text, self.page, self.item_type, self.nocorrect
        )
    }
}

impl YandexMusicClient {
    /// Search text.
    ///
    /// ### Arguments
    /// * `text` - The text to search.
    ///
    /// ### Returns
    /// * [Search] - The search.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn search(
        &self,
        text: String,
    ) -> Result<Search, crate::ClientError> {
        self.search_with(SearchRequest::new(text)).await
    }

    /// Search with optional parameters.
    ///
    /// ### Arguments
    /// * `request` - The request object.
    ///
    /// ### Returns
    /// * [Search] - The search results.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn search_with(
        &self,
        request: SearchRequest,
    ) -> Result<Search, crate::ClientError> {
        let response = self.get(&request.path()).await?;

        Ok(serde_json::from_value::<Search>(response)?)
    }
}
