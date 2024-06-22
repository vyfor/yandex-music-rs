use crate::{
    api::RequestPath,
    model::track_model::similar_tracks::SimilarTracks,
    YandexMusicClient,
};

pub struct SimilarTracksRequest {
    pub track_id: i32,
}

impl SimilarTracksRequest {
    pub fn new(track_id: i32) -> Self {
        Self { track_id }
    }
}

impl RequestPath for SimilarTracksRequest {
    fn path(&self) -> String {
        format!("tracks/{}/similar", self.track_id)
    }
}

impl YandexMusicClient {
    /// Get similar tracks.
    ///
    /// ### Arguments
    /// * `track_id` - The ID of the track.
    ///
    /// ### Returns
    /// * [SimilarTracks] - The similar tracks.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_similar_tracks(
        &self,
        track_id: i32,
    ) -> Result<SimilarTracks, crate::ClientError> {
        let response = self
            .get(&SimilarTracksRequest::new(track_id).path())
            .await?;

        Ok(serde_json::from_value::<SimilarTracks>(response)?)
    }
}
