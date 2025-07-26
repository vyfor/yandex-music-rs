use std::borrow::Cow;

use reqwest::{header::HeaderValue, Method};

use crate::{
    api::Endpoint, client::request::RequestOptions, model::queue::queue_item::QueueItem,
    YandexMusicClient,
};

/// Request for retrieving all available queues.
pub struct GetQueuesOptions {
    /// The device ID in the format: `os=unknown; os_version=unknown; manufacturer=unknown; model=unknown; clid=unknown; device_id=unknown; uuid=unknown`
    pub device_id: String,
}

impl GetQueuesOptions {
    /// Create a new request to get all available queues.
    pub fn new(device_id: impl Into<String>) -> Self {
        Self {
            device_id: device_id.into(),
        }
    }
}

impl Endpoint for GetQueuesOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "queues".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Ok(header_value) = HeaderValue::from_str(&self.device_id) {
            headers.insert("X-Yandex-Music-Device", header_value);
        }
        RequestOptions::default().with_headers(headers)
    }
}

impl YandexMusicClient {
    /// Retrieve all available queues for the specified device.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the device ID.
    ///
    /// ### Returns
    /// * `Result<Vec<QueueItem>, ClientError>` - A list of queue items or an error if the request fails.
    pub async fn get_queues(
        &self,
        options: &GetQueuesOptions,
    ) -> Result<Vec<QueueItem>, crate::ClientError> {
        self.request(options).await
    }
}
