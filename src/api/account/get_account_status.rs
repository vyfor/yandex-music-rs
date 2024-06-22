use crate::{
    api::RequestPath,
    model::account_model::status::AccountStatus,
    YandexMusicClient,
};

pub struct AccountStatusRequest {}

impl RequestPath for AccountStatusRequest {
    fn path(&self) -> String {
        String::from("account/status")
    }
}

impl YandexMusicClient {
    /// Returns user's account status.
    ///
    /// ### Returns
    /// * [AccountStatus] - The user's account status.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_account_status(
        &self,
    ) -> Result<AccountStatus, crate::ClientError> {
        let response =
            self.get(&AccountStatusRequest {}.path()).await?;

        Ok(serde_json::from_value::<AccountStatus>(response)?)
    }
}
