use reqwest::Method;
use std::borrow::Cow;

use crate::{
    api::Endpoint,
    client::request::RequestOptions,
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
    ) -> Result<(), crate::ClientError> {
        self.request::<(), _>(options).await
    }
}
