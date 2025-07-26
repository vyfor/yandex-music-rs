use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::rotor::Rotor, YandexMusicClient,
};

#[derive(Default)]
pub struct GetStationOptions {
    /// The ID of the station to retrieve information for.
    pub station_id: String,
}

impl GetStationOptions {
    /// Create a new request to get station information.
    pub fn new(station_id: impl Into<String>) -> Self {
        Self {
            station_id: station_id.into(),
        }
    }
}

impl Endpoint for GetStationOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("rotor/station/{}/info", self.station_id).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve detailed information about a specific radio station.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the station ID.
    ///
    /// ### Returns
    /// * `Result<Vec<Rotor>, ClientError>` - A list of station information or an error if the request fails.
    pub async fn get_station(
        &self,
        options: &GetStationOptions,
    ) -> Result<Vec<Rotor>, crate::ClientError> {
        self.request(options).await
    }
}
