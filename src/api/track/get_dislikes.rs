use crate::{
    api::{RequestPath, Response},
    error::ClientError,
    model::library::Library,
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
    pub async fn get_disliked_tracks(
        &self,
        user_id: i32,
    ) -> Result<Library, ClientError> {
        let response: Response = self
            .get(&DislikesPlaylistRequest::new(user_id).path())
            .await?;

        Ok(serde_json::from_value::<Library>(response.result)?)
    }
}
