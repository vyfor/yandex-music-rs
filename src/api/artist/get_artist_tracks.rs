use std::borrow::Cow;

use crate::client::request::RequestOptions;
use crate::{api::Endpoint, model::artist::ArtistTracks, YandexMusicClient};
use reqwest::Method;

pub struct ArtistTracksOptions {
    pub artist_id: String,
    pub page: u32,
    pub page_size: u32,
}

impl ArtistTracksOptions {
    pub fn new(artist_id: impl Into<String>) -> Self {
        Self {
            artist_id: artist_id.into(),
            page: 0,
            page_size: 20,
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = page_size;
        self
    }
}

impl Endpoint for ArtistTracksOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!(
            "artists/{}/tracks?page={}&page-size={}",
            self.artist_id, self.page, self.page_size
        )
        .into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Get artist tracks.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the artist ID, page, and page size.
    ///
    /// ### Returns
    /// * `Result<ArtistTracks, ClientError>` - The artist tracks or an error if the request fails.
    pub async fn get_artist_tracks(
        &self,
        options: &ArtistTracksOptions,
    ) -> Result<ArtistTracks, crate::ClientError> {
        self.request::<ArtistTracks, _>(options).await
    }
}
