use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::album::Album, YandexMusicClient,
};

pub struct GetAlbumOptions {
    pub album_id: u32,
    pub with_tracks: bool,
}

impl GetAlbumOptions {
    pub fn new(album_id: u32) -> Self {
        Self {
            album_id,
            with_tracks: false,
        }
    }

    pub fn with_tracks(mut self) -> Self {
        self.with_tracks = true;
        self
    }
}

impl Endpoint for GetAlbumOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!(
            "albums/{}{}",
            self.album_id,
            if self.with_tracks { "/with-tracks" } else { "" }
        )
        .into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Get album.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the album ID and whether to include tracks.
    ///
    /// ### Returns
    /// * `Result<Album, ClientError>` - The album or an error if the request fails.
    pub async fn get_album(&self, options: &GetAlbumOptions) -> Result<Album, crate::ClientError> {
        self.request(options).await
    }
}
