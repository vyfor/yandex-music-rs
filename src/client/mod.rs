//! Client implementation for the Yandex Music API.

pub mod builder;
pub mod request;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::api::Endpoint;
use crate::error::ClientError;
use crate::YandexMusicClient;

use self::request::send_request;

impl YandexMusicClient {
    /// Generic request method that handles all request types.
    pub(crate) async fn request<T, P>(
        &self,
        endpoint: &P,
    ) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
        P: Endpoint,
        <P as Endpoint>::Options: Serialize,
    {
        send_request(&self.inner, endpoint, None).await
    }

    /// Generic request method that handles all request types with a custom url.
    pub(crate) async fn request_with_url<T, P>(
        &self,
        url: String,
        endpoint: &P,
    ) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
        P: Endpoint,
        <P as Endpoint>::Options: Serialize,
    {
        send_request(&self.inner, endpoint, Some(url)).await
    }
}

// Re-export important client types for easier access
pub use self::request::IntoApiParam;
