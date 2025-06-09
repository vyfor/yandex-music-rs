use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::playlist::Playlist, YandexMusicClient,
};

/// Request for changing a playlist's visibility.
pub struct ChangePlaylistVisibilityOptions {
    /// The ID of the user who owns the playlist.
    pub user_id: i32,
    /// The kind (ID) of the playlist.
    pub kind: i32,
    /// The new visibility value ("public" or "private").
    pub value: String,
}

impl ChangePlaylistVisibilityOptions {
    /// Create a new request to change a playlist's visibility.
    pub fn new(user_id: i32, kind: i32, value: impl Into<String>) -> Self {
        Self {
            user_id,
            kind,
            value: value.into(),
        }
    }
}

impl Endpoint for ChangePlaylistVisibilityOptions {
    type Options = [(&'static str, String); 1];
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!("users/{}/playlists/{}/visibility", self.user_id, self.kind).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default().with_form_data([("value", self.value.clone())])
    }
}

impl YandexMusicClient {
    /// Change the visibility of a playlist.
    ///
    /// ### Arguments
    /// * `options` - The request options containing user ID, playlist kind, and visibility value.
    ///
    /// ### Returns
    /// * `Result<Playlist, ClientError>` - The updated playlist or an error if the request fails.
    pub async fn change_playlist_visibility(
        &self,
        options: &ChangePlaylistVisibilityOptions,
    ) -> Result<Playlist, crate::ClientError> {
        self.request::<Playlist, _>(options).await
    }
}
