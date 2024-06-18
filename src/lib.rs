use error::ClientError;
use reqwest::header::{HeaderMap, HeaderValue};

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
}
