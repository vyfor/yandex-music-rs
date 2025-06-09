use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::common::catalogue::Catalogue,
    YandexMusicClient,
};

/// Request for getting the non-music catalogue.
pub struct GetNonMusicCatalogueOptions;

impl Default for GetNonMusicCatalogueOptions {
    fn default() -> Self {
        Self
    }
}

impl Endpoint for GetNonMusicCatalogueOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "non-music/catalogue".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Get the non-music catalogue.
    ///
    /// ### Returns
    /// * `Result<Catalogue, ClientError>` - The non-music catalogue or an error if the request fails.
    pub async fn get_non_music_catalogue(&self) -> Result<Catalogue, crate::ClientError> {
        self.request::<Catalogue, _>(&GetNonMusicCatalogueOptions)
            .await
    }
}
