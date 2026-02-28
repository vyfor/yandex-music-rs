use reqwest::Method;
use std::borrow::Cow;

use crate::{
    api::Endpoint,
    client::request::RequestOptions,
    YandexMusicClient,
};

pub struct RemoveLikedArtistOptions {
    pub user_id: u64,
    pub artist_id: String,
}

impl RemoveLikedArtistOptions {
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

impl Endpoint for RemoveLikedArtistOptions {
    type Options = ();
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!(
            "users/{}/likes/artists/{}/remove",
            self.user_id, self.artist_id
        )
        .into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Remove an artist from the list of liked artists.
    pub async fn remove_liked_artist(
        &self,
        options: &RemoveLikedArtistOptions,
    ) -> Result<(), crate::ClientError> {
        self.request::<(), _>(options).await
    }
}
