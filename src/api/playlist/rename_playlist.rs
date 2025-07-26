use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::playlist::Playlist,
    YandexMusicClient,
};

/// Request for renaming a playlist.
pub struct RenamePlaylistOptions {
    /// The ID of the user who owns the playlist.
    pub user_id: u64,
    /// The kind (ID) of the playlist to rename.
    pub kind: u32,
    /// The new name for the playlist.
    pub value: String,
}

impl RenamePlaylistOptions {
    /// Create a new request to rename a playlist.
    pub fn new(user_id: u64, kind: u32, value: impl Into<String>) -> Self {
        Self {
            user_id,
            kind,
            value: value.into(),
        }
    }
}

impl Endpoint for RenamePlaylistOptions {
    type Options = [(&'static str, String); 1];
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!("users/{}/playlists/{}/name", self.user_id, self.kind).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
            .with_form_data([("value", self.value.clone())])
    }
}

impl YandexMusicClient {
    /// Rename a playlist with a new name.
    ///
    /// ### Arguments
    /// * `options` - The request options containing user ID, playlist kind, and new name.
    ///
    /// ### Returns
    /// * `Result<Playlist, ClientError>` - The updated playlist or an error if the request fails.
    pub async fn rename_playlist(
        &self,
        options: &RenamePlaylistOptions,
    ) -> Result<Playlist, crate::ClientError> {
        self.request(options).await
    }
}
