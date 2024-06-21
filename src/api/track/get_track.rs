use crate::{
    api::{RequestPath, Response},
    model::track::Track,
    YandexMusicClient,
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
        format!("/tracks/{}", self.track_id)
    }
}

impl YandexMusicClient {
    pub async fn get_track(
        &self,
        track_id: i32,
    ) -> Result<Vec<Track>, crate::ClientError> {
        let response: Response =
            self.get(&TrackRequest::new(track_id).path()).await?;

        Ok(serde_json::from_value::<Vec<Track>>(response.result)?)
    }
}
