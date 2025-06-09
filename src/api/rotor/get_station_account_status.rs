use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::account::status::AccountStatus,
    YandexMusicClient,
};

#[derive(Default)]
pub struct GetStationAccountStatusOptions;

impl Endpoint for GetStationAccountStatusOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "rotor/account/status".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve the account status for radio stations.
    ///
    /// ### Returns
    /// * `Result<AccountStatus, ClientError>` - The account status or an error if the request fails.
    pub async fn get_station_account_status(&self) -> Result<AccountStatus, crate::ClientError> {
        self.request::<AccountStatus, _>(&GetStationAccountStatusOptions)
            .await
    }
}
