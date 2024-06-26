use crate::{
    api::RequestPath, model::rotor_model::feedback::StationFeedback,
    YandexMusicClient,
};

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
    /// Send station feedback.
    ///
    /// ### Arguments
    /// * `station_id` - The ID of the station.
    /// * `data` - The feedback data.
    ///
    /// ### Returns
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn send_station_feedback(
        &self,
        station_id: String,
        data: StationFeedback,
    ) -> Result<(), crate::ClientError> {
        self.post_with_json(
            &StationFeedbackRequest::new(station_id).path(),
            serde_json::to_value(data)?,
        )
        .await?;

        Ok(())
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
        .await?;

        Ok(())
    }
}
