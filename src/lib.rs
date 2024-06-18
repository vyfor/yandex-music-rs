use api::{
    playlist::{
        get_all_playlists::AllPlaylistsRequest, get_playlist::PlaylistRequest,
    },
    RequestPath,
};
use error::ClientError;
use model::{download_info::TrackDownloadInfo, playlist::Playlist};
use reqwest::header::{HeaderMap, HeaderValue};

use crate::api::Response;

pub mod api;
pub mod error;
pub mod model;

pub const API_PATH: &str = "https://api.music.yandex.net:443/";

pub struct YandexMusicClient {
    pub client: reqwest::Client,
}

impl YandexMusicClient {
    pub fn new(token: &str) -> Self {
        let mut headers = HeaderMap::with_capacity(1);
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("OAuth {token}"))
                .expect("Failed to set client headers"),
        );

        Self {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Failed to create a Client"),
        }
    }

    async fn get<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, ClientError> {
        let url = format!("{}{}", API_PATH, endpoint);

        let response = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await?;

        Ok(response)
    }

    pub async fn get_all_playlists(
        &self,
        user_id: i32,
    ) -> Result<Vec<Playlist>, ClientError> {
        let response: Response =
            self.get(&AllPlaylistsRequest::new(user_id).path()).await?;

        Ok(serde_json::from_value::<Vec<Playlist>>(response.result)?)
    }

    pub async fn get_playlist(
        &self,
        user_id: i32,
        kind: i32,
    ) -> Result<Playlist, ClientError> {
        let response: Response = self
            .get(&PlaylistRequest::new(user_id, kind).path())
            .await?;

        Ok(serde_json::from_value::<Playlist>(response.result)?)
    }

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
