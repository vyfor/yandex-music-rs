use crate::client::request::RequestOptions;
use crate::{
    api::Endpoint,
    model::search::{Search, SearchType},
    YandexMusicClient,
};
use reqwest::Method;
use std::borrow::Cow;

pub struct SearchOptions {
    pub text: String,
    pub page: u32,
    pub item_type: SearchType,
    pub nocorrect: bool,
}

impl SearchOptions {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            page: 0,
            item_type: SearchType::All,
            nocorrect: false,
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub fn item_type(mut self, item_type: SearchType) -> Self {
        self.item_type = item_type;
        self
    }

    pub fn nocorrect(mut self, nocorrect: bool) -> Self {
        self.nocorrect = nocorrect;
        self
    }
}

impl Endpoint for SearchOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!(
            "search/?text={}&page={}&type={}&nocorrect={}",
            self.text, self.page, self.item_type, self.nocorrect
        )
        .into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Search text.
    ///
    /// ### Arguments
    /// * `options` - The request options containing search text and optional parameters.
    ///
    /// ### Returns
    /// * `Result<Search, ClientError>` - The search results or an error if the request fails.
    pub async fn search(&self, options: &SearchOptions) -> Result<Search, crate::ClientError> {
        self.request(options).await
    }
}
