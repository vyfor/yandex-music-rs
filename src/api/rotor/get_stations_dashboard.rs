use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::rotor::dashboard::StationsDashboard,
    YandexMusicClient,
};

#[derive(Default)]
pub struct GetStationsDashboardOptions;

impl Endpoint for GetStationsDashboardOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "rotor/stations/dashboard".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve the radio stations dashboard with recommended stations.
    ///
    /// ### Returns
    /// * `Result<StationsDashboard, ClientError>` - The stations dashboard or an error if the request fails.
    pub async fn get_stations_dashboard(&self) -> Result<StationsDashboard, crate::ClientError> {
        self.request::<StationsDashboard, _>(&GetStationsDashboardOptions)
            .await
    }
}
