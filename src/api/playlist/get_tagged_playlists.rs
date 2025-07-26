use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::info::tag::TaggedPlaylistIds,
    YandexMusicClient,
};

/// Request for retrieving playlists associated with a specific tag.
pub struct GetTaggedPlaylistsOptions {
    /// The tag name to search for.
    pub tag: String,
}

impl GetTaggedPlaylistsOptions {
    /// Create a new request to get playlists with the specified tag.
    pub fn new(tag: impl Into<String>) -> Self {
        Self { tag: tag.into() }
    }
}

impl Endpoint for GetTaggedPlaylistsOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("tags/{}/playlist-ids", self.tag).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve a list of playlist IDs that have been tagged with the specified tag.
    ///
    /// This endpoint returns a list of playlist IDs that have been associated
    /// with the given tag. Tags are used to categorize playlists by genre, mood,
    /// or other characteristics.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the tag name.
    ///
    /// ### Returns
    /// * `Result<TaggedPlaylistIds, ClientError>` - The tagged playlist IDs or an error if the request fails.
    pub async fn get_tagged_playlist_ids(
        &self,
        options: &GetTaggedPlaylistsOptions,
    ) -> Result<TaggedPlaylistIds, crate::ClientError> {
        self.request(options).await
    }
}
