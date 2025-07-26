use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::rotor::Rotor, YandexMusicClient,
};

/// Request for retrieving all available radio stations.
#[derive(Default)]
pub struct GetAllStationsOptions {
    /// The language (country code) for the station names and descriptions.
    pub language: Option<String>,
}

impl GetAllStationsOptions {
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }
}

impl Endpoint for GetAllStationsOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        if let Some(language) = &self.language {
            format!("rotor/stations/list?language={language}").into()
        } else {
            "rotor/stations/list".into()
        }
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve all available radio stations.
    ///
    /// ### Arguments
    /// * `options` - The request options containing language preference.
    ///
    /// ### Returns
    /// * `Result<Vec<Rotor>, ClientError>` - A list of radio stations or an error if the request fails.
    pub async fn get_all_stations(
        &self,
        options: &GetAllStationsOptions,
    ) -> Result<Vec<Rotor>, crate::ClientError> {
        self.request(options).await
    }
}
