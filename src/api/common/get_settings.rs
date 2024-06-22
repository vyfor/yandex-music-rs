use crate::{
    api::RequestPath,
    model::common_model::settings::Settings,
    YandexMusicClient,
};

pub struct SettingsRequest {}

impl RequestPath for SettingsRequest {
    fn path(&self) -> String {
        String::from("settings")
    }
}

impl YandexMusicClient {
    /// Get settings.
    ///
    /// ### Returns
    /// * [Settings] - The settings.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_settings(&self) -> Result<Settings, crate::ClientError> {
        let response = self.get(&SettingsRequest {}.path()).await?;

        Ok(serde_json::from_value::<Settings>(response)?)
    }
}
