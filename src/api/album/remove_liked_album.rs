use reqwest::Method;
use serde_json::Value;
use std::borrow::Cow;

use crate::{
    api::Endpoint,
    client::request::RequestOptions,
    error::{ClientError, YandexMusicError},
    YandexMusicClient,
};

pub struct RemoveLikedAlbumOptions {
    pub user_id: u64,
    pub album_id: u32,
}

impl RemoveLikedAlbumOptions {
    pub fn new(user_id: u64, album_id: u32) -> Self {
        Self { user_id, album_id }
    }
}

impl Endpoint for RemoveLikedAlbumOptions {
    type Options = ();
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!(
            "users/{}/likes/albums/{}/remove",
            self.user_id, self.album_id
        )
        .into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Remove an album from the list of liked albums.
    pub async fn remove_liked_album(
        &self,
        options: &RemoveLikedAlbumOptions,
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
