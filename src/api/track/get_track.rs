use crate::{
    api::RequestPath, model::track_model::track::Track, YandexMusicClient,
};

pub struct TrackRequest {
    pub track_id: i32,
}

impl TrackRequest {
    pub fn new(track_id: i32) -> Self {
        Self { track_id }
    }
}

impl RequestPath for TrackRequest {
    fn path(&self) -> String {
        format!("tracks/{}", self.track_id)
    }
}

impl YandexMusicClient {
    /// Get track.
    ///
    /// ### Arguments
    /// * `track_id` - The ID of the track.
    ///
    /// ### Returns
    /// * [Track] - The track.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_track(
        &self,
        track_id: i32,
    ) -> Result<Vec<Track>, crate::ClientError> {
        let response = self.get(&TrackRequest::new(track_id).path()).await?;

        Ok(serde_json::from_value::<Vec<Track>>(response)?)
    }
}
