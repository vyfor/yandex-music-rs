use crate::{
    api::{RequestPath, Response},
    model::track_model::supplement::TrackSupplement,
    YandexMusicClient,
};

pub struct TrackSupplementRequest {
    pub track_id: i32,
}

impl TrackSupplementRequest {
    pub fn new(track_id: i32) -> Self {
        Self { track_id }
    }
}

impl RequestPath for TrackSupplementRequest {
    fn path(&self) -> String {
        format!("tracks/{}/supplement", self.track_id)
    }
}

impl YandexMusicClient {
    pub async fn get_track_supplement(
        &self,
        track_id: i32,
    ) -> Result<TrackSupplement, crate::ClientError> {
        let response: Response = self
            .get(&TrackSupplementRequest::new(track_id).path())
            .await?;

        Ok(serde_json::from_value::<TrackSupplement>(response.result)?)
    }
}
