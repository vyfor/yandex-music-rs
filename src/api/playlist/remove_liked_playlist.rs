use reqwest::Method;
use serde_json::Value;
use std::borrow::Cow;

use crate::{
    api::Endpoint,
    client::request::RequestOptions,
    error::{ClientError, YandexMusicError},
    YandexMusicClient,
};

pub struct RemoveLikedPlaylistOptions {
    pub user_id: u64,
    pub owner_uid: u64,
    pub kind: u32,
}

impl RemoveLikedPlaylistOptions {
    pub fn new(user_id: u64, owner_uid: u64, kind: u32) -> Self {
        Self {
            user_id,
            owner_uid,
            kind,
        }
    }
}

impl Endpoint for RemoveLikedPlaylistOptions {
    type Options = ();
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!(
            "users/{}/likes/playlists/{}-{}/remove",
            self.user_id, self.owner_uid, self.kind
        )
        .into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Remove a playlist from the list of liked playlists.
    pub async fn remove_liked_playlist(
        &self,
        options: &RemoveLikedPlaylistOptions,
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
