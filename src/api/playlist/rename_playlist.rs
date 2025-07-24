use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::playlist::Playlist, YandexMusicClient,
    UserId
};

/// Request for renaming a playlist.
pub struct RenamePlaylistOptions<'a> {
    /// The ID of the user who owns the playlist.
    pub user_id: UserId,
    /// The kind (ID) of the playlist to rename.
    pub kind: i32,
    /// The new name for the playlist.
    pub value: &'a str,
}

impl<'a> RenamePlaylistOptions<'a> {
    /// Create a new request to rename a playlist.
    pub fn new(user_id: UserId, kind: i32, value: &'a str) -> Self {
        Self {
            user_id,
            kind,
            value,
        }
    }
}

impl<'a> Endpoint for RenamePlaylistOptions<'a> {
    type Options = [(&'static str, &'a str); 1];
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!("users/{}/playlists/{}/name", self.user_id, self.kind).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default().with_form_data([("value", self.value)])
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
        options: &RenamePlaylistOptions<'_>,
    ) -> Result<Playlist, crate::ClientError> {
        self.request::<Playlist, _>(options).await
    }
}
