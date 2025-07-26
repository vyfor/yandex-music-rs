use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::track::Track, YandexMusicClient,
};

/// Request for retrieving track information.
pub struct GetTrackOptions {
    /// The ID of the track to retrieve.
    pub track_id: String,
}

impl GetTrackOptions {
    /// Create a new request to get track information.
    pub fn new(track_id: impl Into<String>) -> Self {
        Self {
            track_id: track_id.into(),
        }
    }
}

impl Endpoint for GetTrackOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("tracks/{}", self.track_id).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve detailed information about a specific track.
    ///
    /// This endpoint returns metadata about a track, including its title, duration,
    /// artists, album, and availability information.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the track ID.
    ///
    /// ### Returns
    /// * `Result<Vec<Track>, ClientError>` - A vector containing the requested track or an error if the request fails.
    pub async fn get_track(
        &self,
        options: &GetTrackOptions,
    ) -> Result<Vec<Track>, crate::ClientError> {
        self.request(options).await
    }
}
