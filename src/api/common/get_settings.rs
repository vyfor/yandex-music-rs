use crate::{
    api::{RequestPath, Response},
    model::settings::Settings,
    YandexMusicClient,
};

pub struct SettingsRequest {}

impl RequestPath for SettingsRequest {
    fn path(&self) -> String {
        String::from("settings")
    }
}

impl YandexMusicClient {
    pub async fn get_settings(&self) -> Result<Settings, crate::ClientError> {
        let response: Response = self.get(&SettingsRequest {}.path()).await?;

        Ok(serde_json::from_value::<Settings>(response.result)?)
    }
}
