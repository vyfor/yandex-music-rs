use api::Response;
use error::ClientError;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;

pub mod api;
pub mod error;
pub mod model;

pub const API_PATH: &str = "https://api.music.yandex.net:443/";

/// A client to interact with the Yandex Music API.
pub struct YandexMusicClient {
    /// Internal reqwest client.
    pub client: reqwest::Client,
}

impl YandexMusicClient {
    /// Create a new client with a token and default headers.
    pub fn new(token: &str) -> Self {
        let mut headers = HeaderMap::with_capacity(1);
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("OAuth {token}"))
                .expect("Failed to set client headers"),
        );
        headers.insert(
            "X-Yandex-Music-Client",
            HeaderValue::from_str("YandexMusicAndroid/24023621")
                .expect("Failed to set client headers"),
        );

        Self {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Failed to create a Client"),
        }
    }

    /// Create a new client with a token, proxy and default headers.
    pub fn with_proxy(token: &str, proxy: reqwest::Proxy) -> Self {
        let mut headers = HeaderMap::with_capacity(1);
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("OAuth {token}"))
                .expect("Failed to set client headers"),
        );
        headers.insert(
            "X-Yandex-Music-Client",
            HeaderValue::from_str("YandexMusicAndroid/24023621")
                .expect("Failed to set client headers"),
        );

        Self {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .proxy(proxy)
                .build()
                .expect("Failed to create a Client"),
        }
    }

    /// Create a new client with a custom reqwest client.
    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    async fn get(&self, endpoint: &str) -> Result<Value, ClientError> {
        let url = format!("{}{}", API_PATH, endpoint);

        let response: Response = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        if let Some(error) = response.error {
            return Err(error.into());
        }

        Ok(response.result.unwrap())
    }

    async fn get_with_headers(
        &self,
        endpoint: &str,
        headers: HeaderMap,
    ) -> Result<Value, ClientError> {
        let url = format!("{}{}", API_PATH, endpoint);

        let response: Response = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        if let Some(error) = response.error {
            return Err(error.into());
        }

        Ok(response.result.unwrap())
    }

    async fn post(&self, endpoint: &str) -> Result<Value, ClientError> {
        let url = format!("{}{}", API_PATH, endpoint);

        let response: Response = self
            .client
            .post(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        if let Some(error) = response.error {
            return Err(error.into());
        }

        Ok(response.result.unwrap())
    }

    async fn post_with_headers(
        &self,
        endpoint: &str,
        headers: HeaderMap,
    ) -> Result<Value, ClientError> {
        let url = format!("{}{}", API_PATH, endpoint);

        let response: Response = self
            .client
            .post(url)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        if let Some(error) = response.error {
            return Err(error.into());
        }

        Ok(response.result.unwrap())
    }

    async fn post_with_form(
        &self,
        endpoint: &str,
        form: Vec<(&str, String)>,
    ) -> Result<Value, ClientError> {
        let url = format!("{}{}", API_PATH, endpoint);

        let response: Response = self
            .client
            .post(url)
            .form(&form)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        if let Some(error) = response.error {
            return Err(error.into());
        }

        Ok(response.result.unwrap())
    }

    async fn post_with_form_str(
        &self,
        endpoint: &str,
        form: Vec<(&str, &str)>,
    ) -> Result<Value, ClientError> {
        let url = format!("{}{}", API_PATH, endpoint);

        let response: Response = self
            .client
            .post(url)
            .form(&form)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        if let Some(error) = response.error {
            return Err(error.into());
        }

        Ok(response.result.unwrap())
    }

    async fn post_with_json(
        &self,
        endpoint: &str,
        json: serde_json::Value,
    ) -> Result<Value, ClientError> {
        let url = format!("{}{}", API_PATH, endpoint);

        let response: Response = self
            .client
            .post(url)
            .json(&json)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        if let Some(error) = response.error {
            return Err(error.into());
        }

        Ok(response.result.unwrap())
    }
}
