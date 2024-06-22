use crate::{
    api::RequestPath,
    model::account_model::status::AccountStatus,
    YandexMusicClient,
};

pub struct StationAccountStatusRequest {}

impl RequestPath for StationAccountStatusRequest {
    fn path(&self) -> String {
        String::from("rotor/account/status")
    }
}

impl YandexMusicClient {
    /// Get station account status.
    ///
    /// ### Returns
    /// * [AccountStatus] - The station account status.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_station_account_status(
        &self,
    ) -> Result<AccountStatus, crate::ClientError> {
        let response = self.get(&StationAccountStatusRequest {}.path()).await?;

        Ok(serde_json::from_value::<AccountStatus>(response)?)
    }
}
