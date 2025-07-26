use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::rotor::station::StationTracks,
    YandexMusicClient,
};

/// Request for retrieving tracks from a specific radio station.
pub struct GetStationTracksOptions {
    /// The ID of the station to get tracks from.
    pub station_id: String,
    /// Whether to include additional settings in the response.
    pub settings2: bool,
    /// Queue identifier for pagination or continuation.
    pub queue: Option<String>,
}

impl GetStationTracksOptions {
    /// Create a new request to get tracks from a station.
    pub fn new(station_id: impl Into<String>) -> Self {
        Self {
            station_id: station_id.into(),
            settings2: false,
            queue: None,
        }
    }

    /// Set whether to include additional settings in the response.
    pub fn settings2(mut self, settings: bool) -> Self {
        self.settings2 = settings;
        self
    }

    /// Set the queue identifier for pagination or continuation.
    pub fn queue(mut self, queue: impl Into<String>) -> Self {
        self.queue = Some(queue.into());
        self
    }
}

impl Endpoint for GetStationTracksOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        let mut path = format!("rotor/station/{}/tracks", self.station_id);

        if self.settings2 {
            path.push_str("?settings2=true");
        }

        if let Some(queue) = &self.queue {
            path.push_str(&format!("&queue={queue}"));
        }

        path.into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve tracks from a specific radio station.
    ///
    /// ### Arguments
    /// * `options` - The request options containing station ID and query parameters.
    ///
    /// ### Returns
    /// * `Result<StationTracks, ClientError>` - The station tracks or an error if the request fails.
    pub async fn get_station_tracks(
        &self,
        options: &GetStationTracksOptions,
    ) -> Result<StationTracks, crate::ClientError> {
        self.request(options).await
    }
}
