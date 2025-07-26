use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::info::download_info::TrackDownloadInfo,
    YandexMusicClient,
};

pub struct GetDownloadInfoOptions {
    pub track_id: String,
}

impl GetDownloadInfoOptions {
    pub fn new(track_id: impl Into<String>) -> Self {
        Self {
            track_id: track_id.into(),
        }
    }
}

impl Endpoint for GetDownloadInfoOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("tracks/{}/download-info", self.track_id).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Get track download info.
    /// Use [`get_direct_link`](TrackDownloadInfo::get_direct_link) to construct the link to download the track.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the track ID.
    ///
    /// ### Returns
    /// * [`Vec<TrackDownloadInfo>`] - A list of track download info.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_download_info(
        &self,
        options: &GetDownloadInfoOptions,
    ) -> Result<Vec<TrackDownloadInfo>, crate::ClientError> {
        self.request(options).await
    }
}
