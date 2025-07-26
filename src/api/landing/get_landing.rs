use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint,
    client::request::RequestOptions,
    model::landing::{Landing, LandingType},
    YandexMusicClient,
};

pub struct GetLandingOptions {
    /// The blocks to include in the landing page.
    pub blocks: Vec<String>,
}

impl GetLandingOptions {
    /// Create a new request for getting landing page data.
    pub fn new<I>(blocks: I) -> Self
    where
        I: IntoIterator<Item = LandingType>,
    {
        Self {
            blocks: blocks.into_iter().map(|b| b.to_string()).collect(),
        }
    }
}

impl Endpoint for GetLandingOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("landing3?blocks={}", self.blocks.join(",")).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Get landing page data with the specified blocks.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the blocks to include.
    ///
    /// ### Returns
    /// * [Landing] - The landing page data.
    pub async fn get_landing(
        &self,
        options: &GetLandingOptions,
    ) -> Result<Landing, crate::ClientError> {
        self.request(options).await
    }
}
