use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::track::supplement::TrackSupplement,
    YandexMusicClient,
};

/// Request for retrieving supplementary information about a track.
pub struct GetTrackSupplementOptions {
    /// The ID of the track to get supplementary information for.
    pub track_id: String,
}

impl GetTrackSupplementOptions {
    /// Create a new request to get supplementary information for a track.
    pub fn new(track_id: impl Into<String>) -> Self {
        Self {
            track_id: track_id.into(),
        }
    }
}

impl Endpoint for GetTrackSupplementOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("tracks/{}/supplement", self.track_id).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve supplementary information about a track.
    ///
    /// This includes additional metadata like lyrics, videos, and other related content
    /// that isn't part of the basic track information.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the track ID.
    ///
    /// ### Returns
    /// * `Result<TrackSupplement, ClientError>` - The track supplement information or an error if the request fails.
    pub async fn get_track_supplement(
        &self,
        options: &GetTrackSupplementOptions,
    ) -> Result<TrackSupplement, crate::ClientError> {
        self.request::<TrackSupplement, _>(options).await
    }
}
