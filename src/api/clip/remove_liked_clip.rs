use reqwest::Method;
use serde_json::Value;
use std::borrow::Cow;

use crate::{
    api::Endpoint,
    client::request::RequestOptions,
    error::{ClientError, YandexMusicError},
    YandexMusicClient,
};

pub struct RemoveLikedClipOptions {
    pub user_id: u64,
    pub clip_id: u32,
}

impl RemoveLikedClipOptions {
    pub fn new(user_id: u64, clip_id: u32) -> Self {
        Self { user_id, clip_id }
    }
}

impl Endpoint for RemoveLikedClipOptions {
    type Options = ();
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!("users/{}/likes/clips/{}/remove", self.user_id, self.clip_id).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Remove a clip from the list of liked clips.
    pub async fn remove_liked_clip(
        &self,
        options: &RemoveLikedClipOptions,
    ) -> Result<u64, crate::ClientError> {
        let mut response = self.request::<Value, _>(options).await?;

        response["revision"]
            .take()
            .as_u64()
            .ok_or(ClientError::YandexMusicError {
                error: YandexMusicError {
                    name: "InvalidValue".to_string(),
                    message: Some("Revision is not an integer".to_string()),
                },
            })
    }
}
