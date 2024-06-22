use crate::{
    api::RequestPath,
    model::track_model::track::Track,
    YandexMusicClient,
};

pub struct TracksRequest {}

impl RequestPath for TracksRequest {
    fn path(&self) -> String {
        String::from("tracks")
    }
}

impl YandexMusicClient {
    /// Get tracks.
    ///
    /// ### Arguments
    /// * `track_ids` - An array of track IDs.
    /// * `with_positions` - Whether to include track positions in the response.
    ///
    /// ### Returns
    /// * [Vec<Track>] - The tracks.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_tracks(
        &self,
        track_ids: &[i32],
        with_positions: bool,
    ) -> Result<Vec<Track>, crate::ClientError> {
        let response = self
            .post_with_form_str(
                &TracksRequest {}.path(),
                vec![
                    (
                        "track-ids",
                        &track_ids
                            .iter()
                            .map(|&id| id.to_string() + ",")
                            .collect::<String>(),
                    ),
                    ("with-positions", &with_positions.to_string()),
                ],
            )
            .await?;

        Ok(serde_json::from_value::<Vec<Track>>(response)?)
    }
}
