use crate::{
    api::RequestPath,
    model::playlist_model::library::Library,
    YandexMusicClient,
};

pub struct LikesPlaylistRequest {
    pub user_id: i32,
}

impl LikesPlaylistRequest {
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
    }
}

impl RequestPath for LikesPlaylistRequest {
    fn path(&self) -> String {
        format!("/users/{}/likes/tracks", self.user_id)
    }
}

impl YandexMusicClient {
    /// Get liked tracks.
    ///
    /// ### Arguments
    /// * `user_id` - The ID of the user.
    ///
    /// ### Returns
    /// * [Library] - The liked tracks.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_liked_tracks(
        &self,
        user_id: i32,
    ) -> Result<Library, crate::ClientError> {
        let mut response =
            self.get(&LikesPlaylistRequest::new(user_id).path()).await?;

        Ok(serde_json::from_value::<Library>(
            response["library"].take(),
        )?)
    }
}
