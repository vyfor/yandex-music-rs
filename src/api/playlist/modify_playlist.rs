use crate::{
    api::RequestPath,
    model::playlist_model::{modify::ModifyPlaylistDiff, playlist::Playlist},
    YandexMusicClient,
};

pub struct ModifyPlaylistRequest {
    pub user_id: i32,
    pub kind: i32,
}

impl RequestPath for ModifyPlaylistRequest {
    fn path(&self) -> String {
        format!(
            "users/{}/playlists/{}/change-relative",
            self.user_id, self.kind
        )
    }
}

impl ModifyPlaylistRequest {
    pub fn new(user_id: i32, kind: i32) -> Self {
        Self { user_id, kind }
    }
}

impl YandexMusicClient {
    /// Add or remove tracks from playlist.
    ///
    /// ### Arguments
    /// * `user_id` - The ID of the user.
    /// * `kind` - The kind of the playlist.
    /// * `diff` - The diff object of the playlist, indicating the changes.
    /// * `revision` - The revision of the playlist.
    ///
    /// ### Returns
    /// * [Playlist] - The updated playlist.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn modify_playlist(
        &self,
        user_id: i32,
        kind: i32,
        diff: ModifyPlaylistDiff,
        revision: i32,
    ) -> Result<Playlist, crate::ClientError> {
        let response = self
            .post_with_form_str(
                &ModifyPlaylistRequest::new(user_id, kind).path(),
                vec![
                    ("diff", &serde_json::to_string(&diff)?),
                    ("revision", &revision.to_string()),
                ],
            )
            .await?;

        Ok(serde_json::from_value::<Playlist>(response)?)
    }
}
