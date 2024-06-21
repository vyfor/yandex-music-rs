use crate::{
    api::{RequestPath, Response},
    YandexMusicClient,
};

pub struct RemoveLikedTracksRequest {
    pub user_id: i32,
}

impl RemoveLikedTracksRequest {
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
    }
}

impl RequestPath for RemoveLikedTracksRequest {
    fn path(&self) -> String {
        format!("users/{}/likes/tracks/remove", self.user_id)
    }
}

impl YandexMusicClient {
    pub async fn remove_liked_tracks(
        &self,
        user_id: i32,
        track_ids: &[i32],
    ) -> Result<i32, crate::ClientError> {
        let mut response: Response = self
            .post_with_form_str(
                &RemoveLikedTracksRequest::new(user_id).path(),
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
