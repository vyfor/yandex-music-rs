use crate::{api::RequestPath, YandexMusicClient};

pub struct DeletePlaylistRequest {
    pub user_id: i32,
    pub kind: i32,
}

impl RequestPath for DeletePlaylistRequest {
    fn path(&self) -> String {
        format!("users/{}/playlists/{}/delete", self.user_id, self.kind)
    }
}

impl DeletePlaylistRequest {
    pub fn new(user_id: i32, kind: i32) -> Self {
        Self { user_id, kind }
    }
}

impl YandexMusicClient {
    /// Delete playlist.
    ///
    /// ### Arguments
    /// * `user_id` - The ID of the user.
    /// * `kind` - The kind of the playlist.
    ///
    /// ### Returns
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn delete_playlist(
        &self,
        user_id: i32,
        kind: i32,
    ) -> Result<(), crate::ClientError> {
        self.post(&DeletePlaylistRequest::new(user_id, kind).path())
            .await?;

        Ok(())
    }
}
