use reqwest::Method;
use serde_json::Value;
use std::borrow::Cow;

use crate::{
    api::Endpoint,
    client::request::RequestOptions,
    error::{ClientError, YandexMusicError},
    YandexMusicClient,
};

pub struct AddDislikedTracksOptions {
    pub user_id: u64,
    pub track_ids: Vec<String>,
}

impl AddDislikedTracksOptions {
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

impl Endpoint for AddDislikedTracksOptions {
    type Options = [(&'static str, String); 1];
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!("users/{}/dislikes/tracks/add-multiple", self.user_id).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default().with_form_data([("track-ids", self.track_ids.join(","))])
    }
}

impl YandexMusicClient {
    /// Add tracks to the list of disliked tracks.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the user ID and track IDs.
    ///
    /// ### Returns
    /// * `Result<i64, ClientError>` - The revision number of the disliked tracks list or an error if the request fails.
    pub async fn add_disliked_tracks(
        &self,
        options: &AddDislikedTracksOptions,
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
