use crate::{
    api::{RequestPath, Response},
    model::queue_model::queue::Queue,
    YandexMusicClient,
};

pub struct QueueRequest {
    pub queue_id: String,
}

impl QueueRequest {
    pub fn new(queue_id: String) -> Self {
        Self { queue_id }
    }
}

impl RequestPath for QueueRequest {
    fn path(&self) -> String {
        format!("queues/{}", self.queue_id)
    }
}

impl YandexMusicClient {
    pub async fn get_queue(
        &self,
        queue_id: String,
    ) -> Result<Queue, crate::ClientError> {
        let response: Response =
            self.get(&QueueRequest::new(queue_id).path()).await?;

        Ok(serde_json::from_value::<Queue>(response.result)?)
    }
}
