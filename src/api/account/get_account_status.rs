use crate::{
    api::{RequestPath, Response},
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
    pub async fn get_account_status(
        &self,
    ) -> Result<AccountStatus, crate::ClientError> {
        let response: Response =
            self.get(&AccountStatusRequest {}.path()).await?;

        Ok(serde_json::from_value::<AccountStatus>(response.result)?)
    }
}
