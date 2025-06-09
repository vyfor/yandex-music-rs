use std::borrow::Cow;

use reqwest::{header::HeaderValue, Method};

use crate::{
    api::Endpoint, client::request::RequestOptions, model::queue::status::QueueStatus,
    YandexMusicClient,
};

/// Request for updating the current track position in a queue.
pub struct UpdateQueuePositionOptions {
    /// The ID of the queue to update.
    pub queue_id: String,
    /// The current track index in the queue.
    pub current_index: String,
    /// Whether the update is triggered by user interaction.
    pub is_interactive: bool,
    /// The device ID in the format: `os=unknown; os_version=unknown; manufacturer=unknown; model=unknown; clid=unknown; device_id=unknown; uuid=unknown`
    pub device_id: String,
}

impl UpdateQueuePositionOptions {
    /// Create a new request to update the queue position.
    pub fn new(
        queue_id: impl Into<String>,
        current_index: impl Into<String>,
        is_interactive: bool,
        device_id: impl Into<String>,
    ) -> Self {
        Self {
            queue_id: queue_id.into(),
            current_index: current_index.into(),
            is_interactive,
            device_id: device_id.into(),
        }
    }
}

impl Endpoint for UpdateQueuePositionOptions {
    type Options = ();
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!(
            "queues/{}/update-position?currentIndex={}&isInteractive={}",
            self.queue_id, self.current_index, self.is_interactive
        )
        .into()
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
    /// Update the current track position in a queue.
    ///
    /// ### Arguments
    /// * `options` - The request options containing queue ID, current index, interaction flag, and device ID.
    ///
    /// ### Returns
    /// * `Result<QueueStatus, ClientError>` - The updated queue status or an error if the request fails.
    pub async fn update_queue_position(
        &self,
        options: &UpdateQueuePositionOptions,
    ) -> Result<QueueStatus, crate::ClientError> {
        self.request::<QueueStatus, _>(options).await
    }
}
