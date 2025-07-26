use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::track::similar_tracks::SimilarTracks,
    YandexMusicClient,
};

/// Request for retrieving tracks similar to a specific track.
pub struct GetSimilarTracksOptions {
    /// The ID of the track to find similar tracks for.
    pub track_id: String,
}

impl GetSimilarTracksOptions {
    /// Create a new request to find similar tracks.
    pub fn new(track_id: impl Into<String>) -> Self {
        Self {
            track_id: track_id.into(),
        }
    }
}

impl Endpoint for GetSimilarTracksOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("tracks/{}/similar", self.track_id).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve a list of tracks that are similar to the specified track.
    ///
    /// This endpoint uses Yandex Music's recommendation algorithm to find
    /// tracks that are musically similar to the specified track.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the track ID.
    ///
    /// ### Returns
    /// * `Result<SimilarTracks, ClientError>` - The similar tracks or an error if the request fails.
    pub async fn get_similar_tracks(
        &self,
        options: &GetSimilarTracksOptions,
    ) -> Result<SimilarTracks, crate::ClientError> {
        self.request(options).await
    }
}
