use crate::{
    api::RequestPath,
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
    /// Returns user's account settings.
    ///
    /// ### Returns
    /// * [AccountSettings] - The user's account settings.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_account_settings(
        &self,
    ) -> Result<AccountSettings, crate::ClientError> {
        let response =
            self.get(&AccountSettingsRequest {}.path()).await?;

        Ok(serde_json::from_value::<AccountSettings>(response)?)
    }
}
