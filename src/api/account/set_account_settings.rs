use crate::{
    api::{RequestPath, Response},
    model::account_settings::AccountSettings,
    YandexMusicClient,
};

pub struct SetAccountSettingsRequest {}

impl RequestPath for SetAccountSettingsRequest {
    fn path(&self) -> String {
        String::from("account/settings")
    }
}

impl YandexMusicClient {
    pub async fn set_account_settings(
        &self,
        data: AccountSettings,
    ) -> Result<AccountSettings, crate::ClientError> {
        let response: Response = self
            .post_with_json(
                &SetAccountSettingsRequest {}.path(),
                serde_json::to_value(data)?,
            )
            .await?;

        Ok(serde_json::from_value::<AccountSettings>(response.result)?)
    }
}
