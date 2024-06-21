use serde::Serialize;

use crate::{api::RequestPath, YandexMusicClient};

pub struct StationFeedbackRequest {
    pub station_id: String,
    pub batch_id: Option<String>,
}

impl StationFeedbackRequest {
    pub fn new(station_id: String) -> Self {
        Self {
            station_id,
            batch_id: None,
        }
    }

    pub fn with_batch_id(mut self, batch_id: String) -> Self {
        self.batch_id = Some(batch_id);
        self
    }
}

impl RequestPath for StationFeedbackRequest {
    fn path(&self) -> String {
        format!(
            "rotor/station/{}/feedback?batch-id={}",
            self.station_id,
            self.batch_id
                .as_ref()
                .map(|s| s.to_owned())
                .unwrap_or_default()
        )
    }
}

impl YandexMusicClient {
    pub async fn send_station_feedback(
        &self,
        station_id: String,
        data: StationFeedback,
    ) -> Result<(), crate::ClientError> {
        self.post_with_json(
            &StationFeedbackRequest::new(station_id).path(),
            serde_json::to_value(data)?,
        )
        .await
    }

    pub async fn send_station_feedback_with_batch_id(
        &self,
        station_id: String,
        batch_id: String,
        data: StationFeedback,
    ) -> Result<(), crate::ClientError> {
        self.post_with_json(
            &StationFeedbackRequest::new(station_id)
                .with_batch_id(batch_id)
                .path(),
            serde_json::to_value(data)?,
        )
        .await
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StationFeedback {
    pub item_type: String,
    pub timestamp: String,
    pub from: String,
    pub track_id: String,
    pub total_played_seconds: i32,
}
