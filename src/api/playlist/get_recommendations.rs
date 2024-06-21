use crate::{
    api::{RequestPath, Response},
    model::track_model::track::Track,
    YandexMusicClient,
};

pub struct RecommendationsPlaylistRequest {
    pub user_id: i32,
    pub kind: i32,
}

impl RecommendationsPlaylistRequest {
    pub fn new(user_id: i32, kind: i32) -> Self {
        Self { user_id, kind }
    }
}

impl RequestPath for RecommendationsPlaylistRequest {
    fn path(&self) -> String {
        format!(
            "/users/{}/playlists/{}/recommendations",
            self.user_id, self.kind
        )
    }
}

impl YandexMusicClient {
    pub async fn get_recommendations(
        &self,
        user_id: i32,
        kind: i32,
    ) -> Result<Vec<Track>, crate::ClientError> {
        let mut response: Response = self
            .get(&RecommendationsPlaylistRequest::new(user_id, kind).path())
            .await?;

        Ok(serde_json::from_value::<Vec<Track>>(
            response.result["tracks"].take(),
        )?)
    }
}
