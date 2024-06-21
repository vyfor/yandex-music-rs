use crate::{
    api::{RequestPath, Response},
    YandexMusicClient,
};

pub struct AddLikedTracksRequest {
    pub user_id: i32,
}

impl AddLikedTracksRequest {
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
    }
}

impl RequestPath for AddLikedTracksRequest {
    fn path(&self) -> String {
        format!("users/{}/likes/tracks/add-multiple", self.user_id)
    }
}

impl YandexMusicClient {
    pub async fn add_liked_tracks(
        &self,
        user_id: i32,
        track_ids: &[i32],
    ) -> Result<i32, crate::ClientError> {
        let mut response: Response = self
            .post_with_form(
                &AddLikedTracksRequest::new(user_id).path(),
                vec![(
                    "track-ids",
                    &track_ids
                        .iter()
                        .map(|&id| id.to_string() + ",")
                        .collect::<String>(),
                )],
            )
            .await?;

        Ok(serde_json::from_value::<i32>(
            response.result["revision"].take(),
        )?)
    }
}
