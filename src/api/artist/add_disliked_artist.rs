use reqwest::Method;
use std::borrow::Cow;

use crate::{
    api::Endpoint,
    client::request::RequestOptions,
    YandexMusicClient,
};

pub struct AddDislikedArtistOptions {
    pub user_id: u64,
    pub artist_id: String,
}

impl AddDislikedArtistOptions {
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

impl Endpoint for AddDislikedArtistOptions {
    type Options = ();
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!(
            "users/{}/dislikes/artists/add?artist-id={}",
            self.user_id, self.artist_id
        )
        .into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Add an artist to the list of disliked artists.
    pub async fn add_disliked_artist(
        &self,
        options: &AddDislikedArtistOptions,
    ) -> Result<(), crate::ClientError> {
        self.request::<(), _>(options).await
    }
}
