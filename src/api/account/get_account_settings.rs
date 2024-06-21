use crate::{
    api::{RequestPath, Response},
    model::account_model::account_settings::AccountSettings,
    YandexMusicClient,
};

pub struct AccountSettingsRequest {}

impl RequestPath for AccountSettingsRequest {
    fn path(&self) -> String {
        String::from("account/settings")
    }
}

impl YandexMusicClient {
    pub async fn get_account_settings(
        &self,
    ) -> Result<AccountSettings, crate::ClientError> {
        let response: Response =
            self.get(&AccountSettingsRequest {}.path()).await?;

        Ok(serde_json::from_value::<AccountSettings>(response.result)?)
    }
}
