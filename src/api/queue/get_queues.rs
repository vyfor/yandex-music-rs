use reqwest::header::{HeaderMap, HeaderValue};

use crate::{
    api::RequestPath,
    model::queue_model::queue_item::QueueItem,
    YandexMusicClient,
};

pub struct QueuesRequest {}

impl RequestPath for QueuesRequest {
    fn path(&self) -> String {
        String::from("queues")
    }
}

impl YandexMusicClient {
    /// Get queues.
    ///
    /// ### Arguments
    /// * `device_id` - The ID of the device following the format: `os=unknown; os_version=unknown; manufacturer=unknown; model=unknown; clid=unknown; device_id=unknown; uuid=unknown`.
    ///
    /// ### Returns
    /// * [`Vec<QueueItem>`] - The queues.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_queues(
        &self,
        device_id: &str,
    ) -> Result<Vec<QueueItem>, crate::ClientError> {
        let mut headers = HeaderMap::new();
        headers
            .insert("X-Yandex-Music-Device", HeaderValue::from_str(device_id)?);

        let response = self
            .get_with_headers(&QueuesRequest {}.path(), headers)
            .await?;

        Ok(serde_json::from_value::<Vec<QueueItem>>(response)?)
    }
}
