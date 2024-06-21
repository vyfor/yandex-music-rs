use reqwest::header::{HeaderMap, HeaderValue};

use crate::{
    api::{RequestPath, Response},
    model::queue_model::status::QueueStatus,
    YandexMusicClient,
};

pub struct UpdateQueuePositionRequest {
    pub queue_id: String,
    pub current_index: String,
    pub is_interactive: bool,
}

impl UpdateQueuePositionRequest {
    pub fn new(
        queue_id: String,
        current_index: String,
        is_interactive: bool,
    ) -> Self {
        Self {
            queue_id,
            current_index,
            is_interactive,
        }
    }
}

impl RequestPath for UpdateQueuePositionRequest {
    fn path(&self) -> String {
        format!(
            "queues/{}/update-position?currentIndex={}&isInteractive={}",
            self.queue_id, self.current_index, self.is_interactive
        )
    }
}

impl YandexMusicClient {
    pub async fn update_queue_position(
        &self,
        device_id: &str,
        queue_id: String,
        current_index: String,
        is_interactive: bool,
    ) -> Result<QueueStatus, crate::ClientError> {
        let mut headers = HeaderMap::new();
        headers
            .insert("X-Yandex-Music-Device", HeaderValue::from_str(device_id)?);

        let response: Response = self
            .post_with_headers(
                &UpdateQueuePositionRequest::new(
                    queue_id,
                    current_index,
                    is_interactive,
                )
                .path(),
                headers,
            )
            .await?;

        Ok(serde_json::from_value::<QueueStatus>(response.result)?)
    }
}
