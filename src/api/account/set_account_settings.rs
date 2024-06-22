use crate::{
    api::RequestPath,
    model::account_model::account_settings::AccountSettings,
    YandexMusicClient,
};

pub struct SetAccountSettingsRequest {}

impl RequestPath for SetAccountSettingsRequest {
    fn path(&self) -> String {
        String::from("account/settings")
    }
}

impl YandexMusicClient {
    /// Sets user's account settings.
    ///
    /// ### Arguments
    /// * `data` - The account settings to be set.
    ///
    /// ### Returns
    /// * [AccountSettings] - The user's updated account settings.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn set_account_settings(
        &self,
        data: AccountSettings,
    ) -> Result<AccountSettings, crate::ClientError> {
        let response = self
            .post_with_json(
                &SetAccountSettingsRequest {}.path(),
                serde_json::to_value(data)?,
            )
            .await?;

        Ok(serde_json::from_value::<AccountSettings>(response)?)
    }
}
