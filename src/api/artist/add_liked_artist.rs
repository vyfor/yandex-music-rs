use reqwest::Method;
use serde_json::Value;
use std::borrow::Cow;

use crate::{
    api::Endpoint,
    client::request::RequestOptions,
    error::{ClientError, YandexMusicError},
    YandexMusicClient,
};

pub struct AddLikedArtistOptions {
    pub user_id: u64,
    pub artist_id: String,
}

impl AddLikedArtistOptions {
    pub fn new<S>(user_id: u64, artist_id: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            user_id,
            artist_id: artist_id.into(),
        }
    }
}

impl Endpoint for AddLikedArtistOptions {
    type Options = ();
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!(
            "users/{}/likes/artists/add?artist-id={}",
            self.user_id, self.artist_id
        )
        .into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Add an artist to the list of liked artists.
    pub async fn add_liked_artist(
        &self,
        options: &AddLikedArtistOptions,
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
