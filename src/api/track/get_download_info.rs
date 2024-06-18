use crate::{
    api::{self, RequestPath, Response},
    error::ClientError,
    model::download_info::TrackDownloadInfo,
    YandexMusicClient,
};

pub struct DownloadInfoRequest {
    pub track_id: i32,
}

impl DownloadInfoRequest {
    pub fn new(track_id: i32) -> Self {
        Self { track_id }
    }
}

impl RequestPath for DownloadInfoRequest {
    fn path(&self) -> String {
        format!("tracks/{}/download-info", self.track_id)
    }
}

impl YandexMusicClient {
    pub async fn get_track_download_info(
        &self,
        track_id: i32,
    ) -> Result<Vec<TrackDownloadInfo>, ClientError> {
        let response: Response = self
            .get(
                &api::track::get_download_info::DownloadInfoRequest::new(
                    track_id,
                )
                .path(),
            )
            .await?;

        Ok(serde_json::from_value::<Vec<TrackDownloadInfo>>(
            response.result,
        )?)
    }
}
