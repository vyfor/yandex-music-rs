use reqwest::header::{HeaderMap, HeaderValue};

use crate::{
    api::{RequestPath, Response},
    model::queue_item::QueueItem,
    YandexMusicClient,
};

pub struct QueuesRequest {}

impl RequestPath for QueuesRequest {
    fn path(&self) -> String {
        String::from("queues")
    }
}

impl YandexMusicClient {
    pub async fn get_queues(
        &self,
        device_id: &str,
    ) -> Result<Vec<QueueItem>, crate::ClientError> {
        let mut headers = HeaderMap::new();
        headers
            .insert("X-Yandex-Music-Device", HeaderValue::from_str(device_id)?);

        let response: Response = self
            .get_with_headers(&QueuesRequest {}.path(), headers)
            .await?;

        Ok(serde_json::from_value::<Vec<QueueItem>>(response.result)?)
    }
}
