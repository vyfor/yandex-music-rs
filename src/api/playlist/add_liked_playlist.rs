use reqwest::Method;
use std::borrow::Cow;

use crate::{
    api::Endpoint,
    client::request::RequestOptions,
    YandexMusicClient,
};

pub struct AddLikedPlaylistOptions {
    pub user_id: u64,
    pub owner_uid: u64,
    pub kind: u32,
}

impl AddLikedPlaylistOptions {
    pub fn new(user_id: u64, owner_uid: u64, kind: u32) -> Self {
        Self {
            user_id,
            owner_uid,
            kind,
        }
    }
}

impl Endpoint for AddLikedPlaylistOptions {
    type Options = ();
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!(
            "users/{}/likes/playlists/add?owner-uid={}&kind={}",
            self.user_id, self.owner_uid, self.kind
        )
        .into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Add a playlist to the list of liked playlists.
    pub async fn add_liked_playlist(
        &self,
        options: &AddLikedPlaylistOptions,
    ) -> Result<(), crate::ClientError> {
        self.request::<(), _>(options).await
    }
}
