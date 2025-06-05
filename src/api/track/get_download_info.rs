use crate::{
    api::{self, RequestPath},
    model::info_model::download_info::TrackDownloadInfo,
    YandexMusicClient,
};

pub struct DownloadInfoRequest {
    pub track_id: String,
}

impl DownloadInfoRequest {
    pub fn new(track_id: String) -> Self {
        Self { track_id }
    }
}

impl RequestPath for DownloadInfoRequest {
    fn path(&self) -> String {
        format!("tracks/{}/download-info", self.track_id)
    }
}

impl YandexMusicClient {
    /// Get track download info.
    /// Use [`get_direct_link`](TrackDownloadInfo::get_direct_link) to construct the link to download the track.
    ///
    /// ### Arguments
    /// * `track_id` - The ID of the track.
    ///
    /// ### Returns
    /// * [`Vec<TrackDownloadInfo>`] - A list of track download info.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_track_download_info(
        &self,
        track_id: String,
    ) -> Result<Vec<TrackDownloadInfo>, crate::ClientError> {
        let response = self
            .get(
                &api::track::get_download_info::DownloadInfoRequest::new(
                    track_id,
                )
                .path(),
            )
            .await?;

        Ok(serde_json::from_value::<Vec<TrackDownloadInfo>>(response)?)
    }
}
