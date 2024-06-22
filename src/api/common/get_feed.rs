use crate::{
    api::RequestPath,
    model::common_model::feed::Feed,
    YandexMusicClient,
};

pub struct FeedRequest {}

impl RequestPath for FeedRequest {
    fn path(&self) -> String {
        String::from("feed")
    }
}

impl YandexMusicClient {
    /// Get feed.
    ///
    /// ### Returns
    /// * [Feed] - The feed.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_feed(&self) -> Result<Feed, crate::ClientError> {
        let response = self.get(&FeedRequest {}.path()).await?;

        Ok(serde_json::from_value::<Feed>(response)?)
    }
}
