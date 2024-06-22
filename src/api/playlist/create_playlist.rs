use crate::{
    api::RequestPath,
    model::playlist_model::playlist::Playlist,
    YandexMusicClient,
};

pub struct CreatePlaylistRequest {
    pub user_id: i32,
}

impl CreatePlaylistRequest {
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
    }
}

impl RequestPath for CreatePlaylistRequest {
    fn path(&self) -> String {
        format!("users/{}/playlists/create", self.user_id)
    }
}

impl YandexMusicClient {
    /// Create a new playlist.
    ///
    /// ### Arguments
    /// * `user_id` - The ID of the user.
    /// * `title` - The title of the playlist.
    /// * `visibility` - Either `"public"` or `"private"`.
    ///
    /// ### Returns
    /// * [Playlist] - The created playlist.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn create_playlist(
        &self,
        user_id: i32,
        title: &str,
        visibility: &str,
    ) -> Result<Playlist, crate::ClientError> {
        let response = self
            .post_with_form_str(
                &CreatePlaylistRequest::new(user_id).path(),
                vec![("title", title), ("visibility", visibility)],
            )
            .await?;

        Ok(serde_json::from_value::<Playlist>(response)?)
    }
}
