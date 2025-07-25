use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::playlist::Playlist, YandexMusicClient,
};

pub struct GetAllPlaylistsOptions {
    pub user_id: u64,
}

impl GetAllPlaylistsOptions {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

impl Endpoint for GetAllPlaylistsOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("users/{}/playlists/list", self.user_id).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Get all playlists.
    ///
    /// ### Arguments
    /// * `options` - The request options containing user ID.
    ///
    /// ### Returns
    /// * `Result<Vec<Playlist>, ClientError>` - A list of playlists or an error if the request fails.
    pub async fn get_all_playlists(
        &self,
        options: &GetAllPlaylistsOptions,
    ) -> Result<Vec<Playlist>, crate::ClientError> {
        self.request::<Vec<Playlist>, _>(options).await
    }
}
