use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::playlist::Playlist, YandexMusicClient,
};

/// Request for getting a specific playlist.
pub struct GetPlaylistOptions {
    /// The ID of the user who owns the playlist.
    pub user_id: u64,
    /// The kind (ID) of the playlist.
    pub kind: u32,
}

impl GetPlaylistOptions {
    /// Create a new request for getting a specific playlist.
    pub fn new(user_id: u64, kind: u32) -> Self {
        Self { user_id, kind }
    }
}

impl Endpoint for GetPlaylistOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("users/{}/playlists/{}", self.user_id, self.kind).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve a specific playlist by user ID and playlist kind.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the user ID and playlist kind.
    ///
    /// ### Returns
    /// * `Result<Playlist, ClientError>` - The playlist data or an error if the request fails.
    pub async fn get_playlist(
        &self,
        options: &GetPlaylistOptions,
    ) -> Result<Playlist, crate::ClientError> {
        self.request::<Playlist, _>(options).await
    }
}
