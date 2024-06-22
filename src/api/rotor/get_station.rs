use crate::{
    api::{RequestPath, Response},
    model::rotor_model::rotor::Rotor,
    YandexMusicClient,
};

pub struct StationRequest {
    pub station_id: String,
}

impl StationRequest {
    pub fn new(station_id: String) -> Self {
        Self { station_id }
    }
}

impl RequestPath for StationRequest {
    fn path(&self) -> String {
        format!("/rotor/station/{}/info", self.station_id)
    }
}

impl YandexMusicClient {
    pub async fn get_station(
        &self,
        station_id: String,
    ) -> Result<Vec<Rotor>, crate::ClientError> {
        let response: Response =
            self.get(&StationRequest::new(station_id).path()).await?;

        Ok(serde_json::from_value::<Vec<Rotor>>(response.result)?)
    }
}
