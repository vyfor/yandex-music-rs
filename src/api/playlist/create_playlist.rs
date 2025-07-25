use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::playlist::Playlist, YandexMusicClient,
};

/// Request for creating a new playlist.
pub struct CreatePlaylistOptions {
    /// The ID of the user who will own the playlist.
    pub user_id: u64,
    /// The title of the new playlist.
    pub title: String,
    /// The visibility of the playlist. Must be either "public" or "private".
    pub visibility: String,
}

impl CreatePlaylistOptions {
    /// Create a new request for creating a playlist.
    pub fn new(user_id: u64, title: impl Into<String>, visibility: impl Into<String>) -> Self {
        Self {
            user_id,
            title: title.into(),
            visibility: visibility.into(),
        }
    }
}

impl Endpoint for CreatePlaylistOptions {
    type Options = [(&'static str, String); 2];
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!("users/{}/playlists/create", self.user_id).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default().with_form_data([
            ("title", self.title.clone()),
            ("visibility", self.visibility.clone()),
        ])
    }
}

impl YandexMusicClient {
    /// Create a new playlist with the specified title and visibility.
    ///
    /// ### Arguments
    /// * `options` - The request options containing user ID, title, and visibility.
    ///
    /// ### Returns
    /// * `Result<Playlist, ClientError>` - The newly created playlist or an error if the request fails.
    pub async fn create_playlist(
        &self,
        options: &CreatePlaylistOptions,
    ) -> Result<Playlist, crate::ClientError> {
        self.request::<Playlist, _>(options).await
    }
}
