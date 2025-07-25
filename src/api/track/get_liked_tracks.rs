use std::borrow::Cow;

use reqwest::Method;
use serde_json::Value;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::playlist::library::Library,
    YandexMusicClient,
};

/// Request for retrieving a user's liked tracks.
pub struct GetLikedTracksOptions {
    /// The ID of the user whose liked tracks to retrieve.
    pub user_id: u64,
}

impl GetLikedTracksOptions {
    /// Create a new request to get a user's liked tracks.
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

impl Endpoint for GetLikedTracksOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("users/{}/likes/tracks", self.user_id).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve a user's liked tracks.
    ///
    /// This endpoint returns a library of tracks that the specified user has liked.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the user ID.
    ///
    /// ### Returns
    /// * `Result<Library, ClientError>` - The library of liked tracks or an error if the request fails.
    pub async fn get_liked_tracks(
        &self,
        options: &GetLikedTracksOptions,
    ) -> Result<Library, crate::ClientError> {
        let mut response = self.request::<Value, _>(options).await?;

        Ok(serde_json::from_value::<Library>(
            response["library"].take(),
        )?)
    }
}
