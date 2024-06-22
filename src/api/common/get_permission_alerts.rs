use crate::{
    api::RequestPath,
    model::common_model::permission_alerts::PermissionAlerts,
    YandexMusicClient,
};

pub struct PermissionAlertsRequest {}

impl RequestPath for PermissionAlertsRequest {
    fn path(&self) -> String {
        String::from("permission-alerts")
    }
}

impl YandexMusicClient {
    /// Get permission alerts.
    ///
    /// ### Returns
    /// * [PermissionAlerts] - The permission alerts.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_permission_alerts(
        &self,
    ) -> Result<PermissionAlerts, crate::ClientError> {
        let response =
            self.get(&PermissionAlertsRequest {}.path()).await?;

        Ok(serde_json::from_value::<PermissionAlerts>(response)?)
    }
}
