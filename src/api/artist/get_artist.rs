use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::artist::ArtistInfo, YandexMusicClient,
};

pub struct GetArtistOptions {
    pub artist_id: String,
}

impl GetArtistOptions {
    pub fn new(artist_id: impl Into<String>) -> Self {
        Self {
            artist_id: artist_id.into(),
        }
    }
}

impl Endpoint for GetArtistOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("artists/{}", self.artist_id).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Get artist info.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the artist ID.
    ///
    /// ### Returns
    /// * `Result<ArtistInfo, ClientError>` - The artist info or an error if the request fails.
    pub async fn get_artist(
        &self,
        options: &GetArtistOptions,
    ) -> Result<ArtistInfo, crate::ClientError> {
        self.request::<ArtistInfo, _>(options).await
    }
}
