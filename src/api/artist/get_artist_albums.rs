use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint,
    client::request::RequestOptions,
    model::artist::{ArtistAlbums, SortBy},
    YandexMusicClient,
};

pub struct GetArtistAlbumsOptions {
    pub artist_id: String,
    pub page: u32,
    pub page_size: u32,
    pub sort_by: SortBy,
}

impl GetArtistAlbumsOptions {
    pub fn new(artist_id: impl Into<String>) -> Self {
        Self {
            artist_id: artist_id.into(),
            page: 0,
            page_size: 20,
            sort_by: Default::default(),
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

    pub fn sort_by(mut self, sort_by: SortBy) -> Self {
        self.sort_by = sort_by;
        self
    }
}

impl Endpoint for GetArtistAlbumsOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!(
            "artists/{}/direct-albums?page={}&page-size={}&sort-by={}",
            self.artist_id, self.page, self.page_size, self.sort_by
        )
        .into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Get artist albums.
    ///
    /// ### Arguments
    /// * `options` - The request options.
    ///
    /// ### Returns
    /// * [ArtistAlbums] - The artist albums.
    pub async fn get_artist_albums(
        &self,
        options: &GetArtistAlbumsOptions,
    ) -> Result<ArtistAlbums, crate::ClientError> {
        self.request(options).await
    }
}
