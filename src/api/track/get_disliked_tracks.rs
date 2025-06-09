use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::playlist::library::Library,
    YandexMusicClient,
};

/// Request for retrieving a user's disliked tracks.
pub struct GetDislikedTracksOptions {
    /// The ID of the user whose disliked tracks to retrieve.
    pub user_id: i32,
}

impl GetDislikedTracksOptions {
    /// Create a new request to get a user's disliked tracks.
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
    }
}

impl Endpoint for GetDislikedTracksOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("users/{}/dislikes/tracks", self.user_id).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve a user's disliked tracks.
    ///
    /// This endpoint returns a library of tracks that the specified user has marked as disliked.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the user ID.
    ///
    /// ### Returns
    /// * `Result<Library, ClientError>` - The library of disliked tracks or an error if the request fails.
    pub async fn get_disliked_tracks(
        &self,
        options: &GetDislikedTracksOptions,
    ) -> Result<Library, crate::ClientError> {
        self.request::<Library, _>(options).await
    }
}
