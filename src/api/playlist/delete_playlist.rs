use std::borrow::Cow;

use reqwest::Method;

use crate::{api::Endpoint, client::request::RequestOptions, YandexMusicClient};

/// Request for deleting a playlist.
pub struct DeletePlaylistOptions {
    /// The ID of the user who owns the playlist.
    pub user_id: i32,
    /// The kind (ID) of the playlist to delete.
    pub kind: i32,
}

impl DeletePlaylistOptions {
    /// Create a new request for deleting a playlist.
    pub fn new(user_id: i32, kind: i32) -> Self {
        Self { user_id, kind }
    }
}

impl Endpoint for DeletePlaylistOptions {
    type Options = ();
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!("users/{}/playlists/{}/delete", self.user_id, self.kind).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Delete a specific playlist by user ID and playlist kind.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the user ID and playlist kind.
    ///
    /// ### Returns
    /// * `Result<(), ClientError>` - Returns `Ok(())` if the playlist was deleted successfully,
    ///   or an error if the request fails.
    pub async fn delete_playlist(
        &self,
        options: &DeletePlaylistOptions,
    ) -> Result<(), crate::ClientError> {
        self.request::<(), _>(options).await?;

        Ok(())
    }
}
