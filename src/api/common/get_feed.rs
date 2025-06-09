use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::common::feed::Feed, YandexMusicClient,
};

pub struct GetFeedOptions;

impl Default for GetFeedOptions {
    fn default() -> Self {
        Self
    }
}

impl Endpoint for GetFeedOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "feed".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Get the user's feed.
    ///
    /// ### Returns
    /// * `Result<Feed, ClientError>` - The user's feed or an error if the request fails.
    pub async fn get_feed(&self) -> Result<Feed, crate::ClientError> {
        self.request::<Feed, _>(&GetFeedOptions).await
    }
}
