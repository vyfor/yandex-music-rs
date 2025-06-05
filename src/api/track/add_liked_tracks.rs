use crate::{api::RequestPath, YandexMusicClient};

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
    /// Add tracks to the list of liked tracks.
    ///
    /// ### Arguments
    /// * `user_id` - The ID of the user.
    /// * `track_ids` - An array of track IDs to add.
    ///
    /// ### Returns
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn add_liked_tracks(
        &self,
        user_id: i32,
        track_ids: &[String],
    ) -> Result<i32, crate::ClientError> {
        let mut response = self
            .post_with_form_str(
                &AddLikedTracksRequest::new(user_id).path(),
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
