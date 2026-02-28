use reqwest::Method;
use std::borrow::Cow;

use crate::{api::Endpoint, client::request::RequestOptions, YandexMusicClient};

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
    ) -> Result<(), crate::ClientError> {
        self.request::<(), _>(options).await
    }
}
