use crate::{
    api::{RequestPath, Response},
    model::feed::Feed,
    YandexMusicClient,
};

pub struct FeedRequest {}

impl RequestPath for FeedRequest {
    fn path(&self) -> String {
        String::from("feed")
    }
}

impl YandexMusicClient {
    pub async fn get_feed(&self) -> Result<Feed, crate::ClientError> {
        let response: Response = self.get(&FeedRequest {}.path()).await?;

        Ok(serde_json::from_value::<Feed>(response.result)?)
    }
}
