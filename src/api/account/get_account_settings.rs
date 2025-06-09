use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions,
    model::account::account_settings::AccountSettings, YandexMusicClient,
};

#[derive(Default)]
pub struct GetAccountSettingsOptions;

impl Endpoint for GetAccountSettingsOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "account/settings".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Returns user's account settings.
    ///
    /// ### Returns
    /// * `Result<AccountSettings, ClientError>` - The user's account settings or an error if the request fails.
    pub async fn get_account_settings(&self) -> Result<AccountSettings, crate::ClientError> {
        self.request::<AccountSettings, _>(&GetAccountSettingsOptions)
            .await
    }
}
