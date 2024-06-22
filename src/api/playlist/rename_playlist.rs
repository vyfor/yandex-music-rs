use crate::{
    api::RequestPath,
    model::playlist_model::playlist::Playlist,
    YandexMusicClient,
};

pub struct RenamePlaylistRequest {
    pub user_id: i32,
    pub kind: i32,
}

impl RenamePlaylistRequest {
    pub fn new(user_id: i32, kind: i32) -> Self {
        Self { user_id, kind }
    }
}

impl RequestPath for RenamePlaylistRequest {
    fn path(&self) -> String {
        format!("users/{}/playlists/{}/name", self.user_id, self.kind)
    }
}

impl YandexMusicClient {
    /// Rename playlist.
    ///
    /// ### Arguments
    /// * `user_id` - The ID of the user.
    /// * `kind` - The kind of the playlist.
    /// * `value` - The new name of the playlist.
    ///
    /// ### Returns
    /// * [Playlist] - The updated playlist.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn rename_playlist(
        &self,
        user_id: i32,
        kind: i32,
        value: &str,
    ) -> Result<Playlist, crate::ClientError> {
        let response = self
            .post_with_form_str(
                &RenamePlaylistRequest::new(user_id, kind).path(),
                vec![("value", value)],
            )
            .await?;

        Ok(serde_json::from_value::<Playlist>(response)?)
    }
}
