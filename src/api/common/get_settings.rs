use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::common::settings::Settings,
    YandexMusicClient,
};

/// Request for getting the application settings.
pub struct GetSettingsOptions;

impl Default for GetSettingsOptions {
    fn default() -> Self {
        Self
    }
}

impl Endpoint for GetSettingsOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "settings".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Get the current application settings.
    ///
    /// ### Returns
    /// * `Result<Settings, ClientError>` - The application settings or an error if the request fails.
    pub async fn get_settings(&self) -> Result<Settings, crate::ClientError> {
        self.request::<Settings, _>(&GetSettingsOptions).await
    }
}
