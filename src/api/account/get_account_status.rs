use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::account::status::AccountStatus,
    YandexMusicClient,
};

#[derive(Default)]
pub struct GetAccountStatusOptions;

impl Endpoint for GetAccountStatusOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "account/status".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Returns user's account status.
    ///
    /// ### Arguments
    ///
    /// ### Returns
    /// * `Result<AccountStatus, ClientError>` - The user's account status or an error if the request fails.
    pub async fn get_account_status(&self) -> Result<AccountStatus, crate::ClientError> {
        self.request::<AccountStatus, _>(&GetAccountStatusOptions)
            .await
    }
}
