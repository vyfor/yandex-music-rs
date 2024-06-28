use crate::{
    api::RequestPath,
    model::playlist_model::playlist::Playlist,
    YandexMusicClient,
};

pub struct AllPlaylistsRequest {
    pub user_id: i32,
}

impl AllPlaylistsRequest {
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
    }
}

impl RequestPath for AllPlaylistsRequest {
    fn path(&self) -> String {
        format!("users/{}/playlists/list", self.user_id)
    }
}

impl YandexMusicClient {
    /// Get all playlists.
    ///
    /// ### Arguments
    /// * `user_id` - The ID of the user.
    ///
    /// ### Returns
    /// * [`Vec<Playlist>`] - A list of playlists.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_all_playlists(
        &self,
        user_id: i32,
    ) -> Result<Vec<Playlist>, crate::ClientError> {
        let response =
            self.get(&AllPlaylistsRequest::new(user_id).path()).await?;

        Ok(serde_json::from_value::<Vec<Playlist>>(response)?)
    }
}
