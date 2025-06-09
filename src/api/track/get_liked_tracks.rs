use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::playlist::library::Library,
    YandexMusicClient,
};

/// Request for retrieving a user's liked tracks.
pub struct GetLikedTracksOptions {
    /// The ID of the user whose liked tracks to retrieve.
    pub user_id: i32,
}

impl GetLikedTracksOptions {
    /// Create a new request to get a user's liked tracks.
    pub fn new(user_id: i32) -> Self {
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
        self.request::<Library, _>(options).await
    }
}
