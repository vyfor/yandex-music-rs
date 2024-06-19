use crate::{
    api::{RequestPath, Response},
    model::status::AccountStatus,
    YandexMusicClient,
};

pub struct StationAccountStatusRequest {}

impl RequestPath for StationAccountStatusRequest {
    fn path(&self) -> String {
        String::from("rotor/account/status")
    }
}

impl YandexMusicClient {
    pub async fn get_station_account_status(
        &self,
    ) -> Result<AccountStatus, crate::ClientError> {
        let response: Response =
            self.get(&StationAccountStatusRequest {}.path()).await?;

        Ok(serde_json::from_value::<AccountStatus>(response.result)?)
    }
}
