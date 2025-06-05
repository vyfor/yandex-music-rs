use crate::{
    api::RequestPath, model::track_model::supplement::TrackSupplement,
    YandexMusicClient,
};

pub struct TrackSupplementRequest {
    pub track_id: String,
}

impl TrackSupplementRequest {
    pub fn new(track_id: String) -> Self {
        Self { track_id }
    }
}

impl RequestPath for TrackSupplementRequest {
    fn path(&self) -> String {
        format!("tracks/{}/supplement", self.track_id)
    }
}

impl YandexMusicClient {
    /// Get additional information about the track.
    ///
    /// ### Arguments
    /// * `track_id` - The ID of the track.
    ///
    /// ### Returns
    /// * [TrackSupplement] - The track supplement.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_track_supplement(
        &self,
        track_id: String,
    ) -> Result<TrackSupplement, crate::ClientError> {
        let response = self
            .get(&TrackSupplementRequest::new(track_id).path())
            .await?;

        Ok(serde_json::from_value::<TrackSupplement>(response)?)
    }
}
