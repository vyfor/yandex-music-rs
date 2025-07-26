use std::borrow::Cow;

use reqwest::Method;
use serde_json::Value;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::account::settings::AccountSettings,
    YandexMusicClient,
};

pub struct SetAccountSettingsOptions {
    pub data: AccountSettings,
}

impl SetAccountSettingsOptions {
    pub fn new(data: AccountSettings) -> Self {
        Self { data }
    }
}

impl Endpoint for SetAccountSettingsOptions {
    type Options = Value;
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        "account/settings".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default().with_json_data(serde_json::to_value(&self.data).unwrap())
    }
}

impl YandexMusicClient {
    /// Sets user's account settings.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the account settings to be set.
    ///
    /// ### Returns
    /// * `Result<AccountSettings, ClientError>` - The user's updated account settings or an error if the request fails.
    pub async fn set_account_settings(
        &self,
        options: &SetAccountSettingsOptions,
    ) -> Result<AccountSettings, crate::ClientError> {
        self.request(options).await
    }
}
