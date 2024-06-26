use crate::{
    api::RequestPath, model::rotor_model::dashboard::StationsDashboard,
    YandexMusicClient,
};

pub struct StationsDashboardRequest {}

impl RequestPath for StationsDashboardRequest {
    fn path(&self) -> String {
        String::from("/rotor/stations/dashboard")
    }
}

impl YandexMusicClient {
    pub async fn get_stations_dashboard(
        &self,
    ) -> Result<StationsDashboard, crate::ClientError> {
        let response = self.get(&StationsDashboardRequest {}.path()).await?;

        Ok(serde_json::from_value::<StationsDashboard>(response)?)
    }
}
