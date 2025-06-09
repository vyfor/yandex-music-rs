use std::{borrow::Cow, collections::HashMap};

use reqwest::Method;

use crate::{api::Endpoint, client::request::RequestOptions, YandexMusicClient};

#[derive(Default)]
pub struct GetAccountExperimentsOptions;

impl Endpoint for GetAccountExperimentsOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "account/experiments".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Returns a list of user's account experiments.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the user ID.
    ///
    /// ### Returns
    /// * `Result<HashMap<String, String>, ClientError>` - The list of user's account experiments or an error if the request fails.
    pub async fn get_account_experiments(
        &self,
    ) -> Result<HashMap<String, String>, crate::ClientError> {
        self.request::<HashMap<String, String>, _>(&GetAccountExperimentsOptions)
            .await
    }
}
