use reqwest::Method;
use std::borrow::Cow;

use crate::{
    api::Endpoint,
    client::request::RequestOptions,
    YandexMusicClient,
};

pub struct AddLikedAlbumOptions {
    pub user_id: u64,
    pub album_id: u32,
}

impl AddLikedAlbumOptions {
    pub fn new(user_id: u64, album_id: u32) -> Self {
        Self { user_id, album_id }
    }
}

impl Endpoint for AddLikedAlbumOptions {
    type Options = ();
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!(
            "users/{}/likes/albums/add?album-id={}",
            self.user_id, self.album_id
        )
        .into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Add an album to the list of liked albums.
    pub async fn add_liked_album(
        &self,
        options: &AddLikedAlbumOptions,
    ) -> Result<(), crate::ClientError> {
        self.request::<(), _>(options).await
    }
}
