use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::track::Track, YandexMusicClient,
};

/// Request for retrieving multiple tracks by their IDs.
pub struct GetTracksOptions {
    /// Array of track IDs to retrieve.
    pub track_ids: Vec<String>,
    /// Whether to include track positions in the response.
    pub with_positions: bool,
}

impl GetTracksOptions {
    /// Create a new request to get multiple tracks.
    pub fn new<I, S>(track_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            track_ids: track_ids.into_iter().map(|s| s.into()).collect(),
            with_positions: false,
        }
    }

    /// Set whether to include track positions in the response.
    pub fn with_positions(mut self, with_positions: bool) -> Self {
        self.with_positions = with_positions;
        self
    }
}

impl Endpoint for GetTracksOptions {
    type Options = [(&'static str, String); 2];
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        "tracks".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        let track_ids = self.track_ids.join(",") + ",";
        RequestOptions::default().with_form_data([
            ("track-ids", track_ids),
            ("with-positions", self.with_positions.to_string()),
        ])
    }
}

impl YandexMusicClient {
    /// Retrieve multiple tracks by their IDs in a single request.
    ///
    /// This endpoint is more efficient than making individual requests for each track
    /// when you need to fetch multiple tracks at once.
    ///
    /// ### Arguments
    /// * `options` - The request options containing track IDs and position flag.
    ///
    /// ### Returns
    /// * `Result<Vec<Track>, ClientError>` - A vector of the requested tracks or an error if the request fails.
    pub async fn get_tracks(
        &self,
        options: &GetTracksOptions,
    ) -> Result<Vec<Track>, crate::ClientError> {
        self.request(options).await
    }
}
