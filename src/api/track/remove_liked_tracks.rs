use crate::{api::RequestPath, YandexMusicClient};

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
    /// Remove tracks from the list of liked tracks.
    ///
    /// ### Arguments
    /// * `user_id` - The ID of the user.
    /// * `track_ids` - An array of track IDs to remove.
    ///
    /// ### Returns
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn remove_liked_tracks(
        &self,
        user_id: i32,
        track_ids: &[String],
    ) -> Result<i32, crate::ClientError> {
        let mut response = self
            .post_with_form_str(
                &RemoveLikedTracksRequest::new(user_id).path(),
                vec![(
                    "track-ids",
                    &track_ids
                        .iter()
                        .map(|id| id.to_string() + ",")
                        .collect::<String>(),
                )],
            )
            .await?;

        Ok(serde_json::from_value::<i32>(response["revision"].take())?)
    }
}
