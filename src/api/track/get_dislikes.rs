use crate::{
    api::RequestPath,
    model::playlist_model::library::Library,
    YandexMusicClient,
};

pub struct DislikesPlaylistRequest {
    pub user_id: i32,
}

impl DislikesPlaylistRequest {
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
    }
}

impl RequestPath for DislikesPlaylistRequest {
    fn path(&self) -> String {
        format!("/users/{}/dislikes/tracks", self.user_id)
    }
}

impl YandexMusicClient {
    /// Get disliked tracks.
    ///
    /// ### Arguments
    /// * `user_id` - The ID of the user.
    ///
    /// ### Returns
    /// * [Library] - The disliked tracks.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_disliked_tracks(
        &self,
        user_id: i32,
    ) -> Result<Library, crate::ClientError> {
        let mut response = self
            .get(&DislikesPlaylistRequest::new(user_id).path())
            .await?;

        Ok(serde_json::from_value::<Library>(
            response["library"].take(),
        )?)
    }
}
