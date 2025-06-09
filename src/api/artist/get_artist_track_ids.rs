use crate::client::request::RequestOptions;
use crate::{api::Endpoint, model::artist::ArtistTrackIds, YandexMusicClient};
use reqwest::Method;
use std::borrow::Cow;

pub struct ArtistTrackIdsOptions {
    pub artist_id: String,
}

impl ArtistTrackIdsOptions {
    pub fn new(artist_id: impl Into<String>) -> Self {
        Self {
            artist_id: artist_id.into(),
        }
    }
}

impl Endpoint for ArtistTrackIdsOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("artists/{}/track-ids-by-rating", self.artist_id).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Get artist track ids.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the artist ID.
    ///
    /// ### Returns
    /// * `Result<ArtistTrackIds, ClientError>` - The artist track ids or an error if the request fails.
    pub async fn get_artist_track_ids(
        &self,
        options: &ArtistTrackIdsOptions,
    ) -> Result<ArtistTrackIds, crate::ClientError> {
        self.request::<ArtistTrackIds, _>(options).await
    }
}
