use std::borrow::Cow;

use reqwest::Method;
use serde::Deserialize;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::landing::wave::LandingWave,
    YandexMusicClient,
};

#[derive(Default)]
pub struct GetWavesOptions;

impl Endpoint for GetWavesOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "landing-blocks/waves".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

#[derive(Deserialize)]
struct GetWavesResponse {
    waves: Vec<LandingWave>,
}

impl YandexMusicClient {
    /// Get waves/stations information.
    ///
    /// ### Returns
    /// * [Vec<Wave>] - The landing waves data.
    pub async fn get_waves(&self) -> Result<Vec<LandingWave>, crate::ClientError> {
        self.request_direct::<GetWavesResponse, _>(&GetWavesOptions::default())
            .await
            .map(|res| res.waves)
    }
}
