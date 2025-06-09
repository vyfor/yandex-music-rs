use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::queue::Queue, YandexMusicClient,
};

/// Request for getting a queue by its ID.
pub struct GetQueueOptions {
    /// The ID of the queue to retrieve.
    pub queue_id: String,
}

impl GetQueueOptions {
    /// Create a new request to get a queue by its ID.
    pub fn new(queue_id: impl Into<String>) -> Self {
        Self {
            queue_id: queue_id.into(),
        }
    }
}

impl Endpoint for GetQueueOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("queues/{}", self.queue_id).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve a queue by its ID.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the queue ID.
    ///
    /// ### Returns
    /// * `Result<Queue, ClientError>` - The queue or an error if the request fails.
    pub async fn get_queue(&self, options: &GetQueueOptions) -> Result<Queue, crate::ClientError> {
        self.request::<Queue, _>(options).await
    }
}
