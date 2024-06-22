use crate::{
    api::{RequestPath, Response},
    model::rotor_model::station::StationTracks,
    YandexMusicClient,
};

pub struct StationTracksRequest {
    pub station_id: String,
    pub settings2: bool,
    pub queue: String,
}

impl StationTracksRequest {
    pub fn new(station_id: String) -> Self {
        Self {
            station_id,
            settings2: true,
            queue: String::new(),
        }
    }
}

impl RequestPath for StationTracksRequest {
    fn path(&self) -> String {
        format!("/rotor/station/{}/tracks", self.station_id)
    }
}

impl YandexMusicClient {
    pub async fn get_station_tracks(
        &self,
        station_id: String,
    ) -> Result<StationTracks, crate::ClientError> {
        self.get_station_tracks_with(&StationTracksRequest::new(station_id))
            .await
    }

    pub async fn get_station_tracks_with(
        &self,
        request: &StationTracksRequest,
    ) -> Result<StationTracks, crate::ClientError> {
        let response: Response = self.get(&request.path()).await?;

        Ok(serde_json::from_value::<StationTracks>(response.result)?)
    }
}
