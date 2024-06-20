use crate::{
    api::{RequestPath, Response},
    model::permission_alerts::PermissionAlerts,
    YandexMusicClient,
};

pub struct PermissionAlertsRequest {}

impl RequestPath for PermissionAlertsRequest {
    fn path(&self) -> String {
        String::from("permission-alerts")
    }
}

impl YandexMusicClient {
    pub async fn get_permission_alerts(
        &self,
    ) -> Result<PermissionAlerts, crate::ClientError> {
        let response: Response =
            self.get(&PermissionAlertsRequest {}.path()).await?;

        Ok(serde_json::from_value::<PermissionAlerts>(response.result)?)
    }
}
