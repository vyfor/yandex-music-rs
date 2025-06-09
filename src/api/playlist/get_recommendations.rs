use crate::client::request::RequestOptions;
use crate::{api::Endpoint, model::track::Track, YandexMusicClient};
use reqwest::Method;
use serde_json::Value;
use std::borrow::Cow;

pub struct GetRecommendationsOptions {
    pub user_id: i32,
    pub kind: i32,
}

impl GetRecommendationsOptions {
    pub fn new(user_id: i32, kind: i32) -> Self {
        Self { user_id, kind }
    }
}

impl Endpoint for GetRecommendationsOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!(
            "/users/{}/playlists/{}/recommendations",
            self.user_id, self.kind
        )
        .into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Get track recommendations.
    ///
    /// ### Arguments
    /// * `options` - The request options containing user ID and playlist kind.
    ///
    /// ### Returns
    /// * `Result<Vec<Track>, ClientError>` - A list of tracks or an error if the request fails.
    pub async fn get_recommendations(
        &self,
        options: &GetRecommendationsOptions,
    ) -> Result<Vec<Track>, crate::ClientError> {
        let mut response = self.request::<Value, _>(options).await?;

        Ok(serde_json::from_value::<Vec<Track>>(
            response["tracks"].take(),
        )?)
    }
}
