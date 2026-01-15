use std::borrow::Cow;

use reqwest::Method;
use serde_json::Value;

use crate::{
    api::Endpoint,
    client::request::RequestOptions,
    error::{ClientError, YandexMusicError},
    YandexMusicClient,
};

pub struct RemoveDislikedTracksOptions {
    pub user_id: u64,
    pub track_ids: Vec<String>,
}

impl RemoveDislikedTracksOptions {
    pub fn new<S, I>(user_id: u64, track_ids: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        Self {
            user_id,
            track_ids: track_ids.into_iter().map(|s| s.into()).collect(),
        }
    }
}

impl Endpoint for RemoveDislikedTracksOptions {
    type Options = [(&'static str, String); 1];
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!("users/{}/dislikes/tracks/remove", self.user_id).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default().with_form_data([("track-ids", self.track_ids.join(","))])
    }
}

impl YandexMusicClient {
    /// Remove tracks from the list of disliked tracks.
    ///
    /// ### Arguments
    /// * `options` - The request options containing user ID and track IDs.
    ///
    /// ### Returns
    /// * `Result<i32, ClientError>` - The revision number of the updated list or an error if the request fails.
    pub async fn remove_disliked_tracks(
        &self,
        options: &RemoveDislikedTracksOptions,
    ) -> Result<u64, crate::ClientError> {
        let mut response = self.request::<Value, _>(options).await?;

        response["revision"]
            .take()
            .as_u64()
            .ok_or(ClientError::YandexMusicError {
                error: YandexMusicError {
                    name: "InvalidValue".to_string(),
                    message: Some("Revision is not an integer".to_string()),
                },
            })
    }
}
