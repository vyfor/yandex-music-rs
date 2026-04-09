use std::borrow::Cow;

use reqwest::Method;
use serde_json::to_value;

use crate::{
    api::Endpoint, client::request::RequestOptions,
    model::rotor::feedback::StationFeedback, YandexMusicClient,
};

/// Request for sending feedback about a radio station's track.
pub struct GetStationFeedbackOptions {
    /// The ID of the station to provide feedback for.
    pub station_id: String,
    /// Optional batch ID for grouping related feedback.
    pub batch_id: Option<String>,
    /// The feedback data containing track information and user action.
    pub feedback: StationFeedback,
}

impl GetStationFeedbackOptions {
    /// Create a new feedback request for a station.
    pub fn new(
        station_id: impl Into<String>,
        feedback: StationFeedback,
    ) -> Self {
        Self {
            station_id: station_id.into(),
            batch_id: None,
            feedback,
        }
    }

    /// Set the batch ID for grouping related feedback.
    pub fn batch_id(mut self, batch_id: impl Into<String>) -> Self {
        self.batch_id = Some(batch_id.into());
        self
    }
}

impl Endpoint for GetStationFeedbackOptions {
    type Options = ();
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        if let Some(batch_id) = &self.batch_id {
            format!(
                "rotor/station/{}/feedback?batch-id={}",
                self.station_id, batch_id
            )
            .into()
        } else {
            format!("rotor/station/{}/feedback", self.station_id).into()
        }
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
            .with_json_data(serde_json::to_value(&self.feedback).unwrap())
    }
}

impl YandexMusicClient {
    /// Send feedback about a track played on a radio station.
    ///
    /// This is used to provide feedback to the recommendation algorithm
    /// about user interactions with tracks (like/dislike, skip, etc.).
    ///
    /// ### Arguments
    /// * `options` - The request options containing station ID, feedback data, and optional batch ID.
    ///
    /// ### Returns
    /// * `Result<(), ClientError>` - An empty result or an error if the request fails.
    pub async fn send_station_feedback(
        &self,
        options: &GetStationFeedbackOptions,
    ) -> Result<(), crate::ClientError> {
        let url = if let Some(batch_id) = &options.batch_id {
            format!(
                "https://api.music.yandex.net/rotor/station/{}/feedback?batch-id={}",
                options.station_id, batch_id
            )
        } else {
            format!(
                "https://api.music.yandex.net/rotor/station/{}/feedback",
                options.station_id
            )
        };

        let response = self
            .inner
            .post(url)
            .json(&to_value(&options.feedback)?)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(crate::error::ClientError::YandexMusicError {
                error: crate::error::YandexMusicError {
                    name: "RequestFailed".to_string(),
                    message: Some(format!(
                        "Request failed with status code: {}",
                        response.status()
                    )),
                },
            });
        }

        Ok(())
    }
}
