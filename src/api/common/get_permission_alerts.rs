use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions,
    model::common::permission_alerts::PermissionAlerts, YandexMusicClient,
};

/// Request for getting permission alerts.
pub struct GetPermissionAlertsOptions;

impl Default for GetPermissionAlertsOptions {
    fn default() -> Self {
        Self
    }
}

impl Endpoint for GetPermissionAlertsOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "permission-alerts".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Get permission alerts for the current user.
    ///
    /// ### Returns
    /// * `Result<PermissionAlerts, ClientError>` - The permission alerts or an error if the request fails.
    pub async fn get_permission_alerts(&self) -> Result<PermissionAlerts, crate::ClientError> {
        self.request::<PermissionAlerts, _>(&GetPermissionAlertsOptions)
            .await
    }
}
