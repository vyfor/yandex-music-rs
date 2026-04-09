use std::time::Duration;

use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Serialize, Serializer};

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StationFeedback {
    #[serde(rename = "type")]
    pub item_type: String,
    #[serde(serialize_with = "serialize_timestamp_millis")]
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_id: Option<String>,
    #[serde(
        rename = "totalPlayedSeconds",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_optional_duration_to_secs"
    )]
    pub total_played: Option<Duration>,
}

fn serialize_timestamp_millis<S>(
    value: &DateTime<Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_rfc3339_opts(SecondsFormat::Millis, true))
}

impl StationFeedback {
    pub fn new(item_type: impl Into<String>) -> Self {
        Self {
            item_type: item_type.into(),
            timestamp: Utc::now(),
            from: None,
            track_id: None,
            total_played: None,
        }
    }

    pub fn radio_started(from: impl Into<String>) -> Self {
        Self::new("radioStarted").with_from(from)
    }

    pub fn track_started(track_id: impl Into<String>, from: impl Into<String>) -> Self {
        Self::new("trackStarted")
            .with_track_id(track_id)
            .with_from(from)
    }

    pub fn track_finished(
        track_id: impl Into<String>,
        from: impl Into<String>,
        total_played: Duration,
    ) -> Self {
        Self::new("trackFinished")
            .with_track_id(track_id)
            .with_from(from)
            .with_total_played(total_played)
    }

    pub fn skip(
        track_id: impl Into<String>,
        from: impl Into<String>,
        total_played: Duration,
    ) -> Self {
        Self::new("skip")
            .with_track_id(track_id)
            .with_from(from)
            .with_total_played(total_played)
    }

    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn with_from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn with_track_id(mut self, track_id: impl Into<String>) -> Self {
        self.track_id = Some(track_id.into());
        self
    }

    pub fn with_total_played(mut self, total_played: Duration) -> Self {
        self.total_played = Some(total_played);
        self
    }
}

fn serialize_optional_duration_to_secs<S>(
    value: &Option<Duration>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(duration) => serializer.serialize_f64(duration.as_secs_f64()),
        None => serializer.serialize_none(),
    }
}
