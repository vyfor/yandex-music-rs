use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::common::genre::Genre, YandexMusicClient,
};

/// Request for getting a list of all available genres.
pub struct GetGenresOptions;

impl Default for GetGenresOptions {
    fn default() -> Self {
        Self
    }
}

impl Endpoint for GetGenresOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "genres".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Get a list of all available genres.
    ///
    /// ### Returns
    /// * `Result<Vec<Genre>, ClientError>` - A list of genres or an error if the request fails.
    pub async fn get_genres(&self) -> Result<Vec<Genre>, crate::ClientError> {
        self.request::<Vec<Genre>, _>(&GetGenresOptions).await
    }
}
