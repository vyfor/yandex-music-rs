use crate::{
    api::{RequestPath, Response},
    model::rotor_model::rotor::Rotor,
    YandexMusicClient,
};

pub struct AllStationsRequest {
    pub language: String,
}

impl AllStationsRequest {
    pub fn new() -> Self {
        Self {
            language: String::from("en"),
        }
    }

    pub fn language(mut self, language: String) -> Self {
        self.language = language;

        self
    }
}

impl Default for AllStationsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestPath for AllStationsRequest {
    fn path(&self) -> String {
        String::from("/rotor/stations/list")
    }
}

impl YandexMusicClient {
    pub async fn get_all_stations(
        &self,
    ) -> Result<Vec<Rotor>, crate::ClientError> {
        let response: Response =
            self.get(&AllStationsRequest::new().path()).await?;

        Ok(serde_json::from_value::<Vec<Rotor>>(response.result)?)
    }

    pub async fn get_all_stations_with_language(
        &self,
        language: String,
    ) -> Result<Vec<Rotor>, crate::ClientError> {
        let response: Response = self
            .get(&AllStationsRequest::new().language(language).path())
            .await?;

        Ok(serde_json::from_value::<Vec<Rotor>>(response.result)?)
    }
}
