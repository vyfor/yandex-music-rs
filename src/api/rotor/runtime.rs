use std::time::Duration;

use crate::{
    error::{ClientError, YandexMusicError},
    model::{
        rotor::{
            feedback::StationFeedback,
            session::Session,
        },
        track::Track,
    },
    YandexMusicClient,
};

use super::{
    create_session::CreateSessionOptions,
    get_station_tracks::GetStationTracksOptions,
    send_station_feedback::GetStationFeedbackOptions,
};

const DEFAULT_HISTORY_LIMIT: usize = 20;

#[derive(Debug, Clone, PartialEq)]
pub struct RotorSessionRuntime {
    pub session: Session,
    pub station_id: String,
    pub from: String,
    pub history: Vec<String>,
    history_limit: usize,
}

impl RotorSessionRuntime {
    pub fn from_session(session: Session) -> Result<Self, ClientError> {
        let station_id = session
            .wave
            .as_ref()
            .map(|wave| wave.station_id.clone())
            .or_else(|| session.radio_session_id.clone())
            .ok_or_else(missing_runtime_context)?;

        let from = session
            .wave
            .as_ref()
            .map(|wave| wave.id_for_from.clone())
            .unwrap_or_else(|| "rotor".to_string());

        Ok(Self {
            session,
            station_id,
            from,
            history: Vec::new(),
            history_limit: DEFAULT_HISTORY_LIMIT,
        })
    }

    pub fn batch_id(&self) -> &str {
        &self.session.batch_id
    }

    pub fn with_history_limit(mut self, limit: usize) -> Self {
        self.history_limit = limit.max(1);
        self
    }

    pub fn record_track(&mut self, track: &Track) -> Option<String> {
        let seed = track_seed(track)?;
        self.history.insert(0, seed.clone());
        self.history.truncate(self.history_limit);
        Some(seed)
    }

    pub fn current_track_id(&self) -> Option<&str> {
        self.history
            .first()
            .and_then(|seed| seed.split(':').next())
    }

    pub fn feedback(&self, item_type: impl Into<String>) -> StationFeedback {
        StationFeedback::new(item_type).with_from(self.from.clone())
    }

    pub fn radio_started_feedback(&self) -> StationFeedback {
        StationFeedback::radio_started(self.from.clone())
    }

    pub fn track_started_feedback(&self, track: &Track) -> StationFeedback {
        StationFeedback::track_started(track.id.clone(), self.from.clone())
    }

    pub fn track_finished_feedback(
        &self,
        track: &Track,
        total_played: Duration,
    ) -> StationFeedback {
        StationFeedback::track_finished(track.id.clone(), self.from.clone(), total_played)
    }

    pub fn skip_feedback(&self, track: &Track, total_played: Duration) -> StationFeedback {
        StationFeedback::skip(track.id.clone(), self.from.clone(), total_played)
    }
}

impl YandexMusicClient {
    pub async fn start_rotor_session<I, S>(
        &self,
        seeds: I,
    ) -> Result<RotorSessionRuntime, ClientError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let session = self.create_session(CreateSessionOptions::new(seeds)).await?;
        RotorSessionRuntime::from_session(session)
    }

    pub async fn continue_rotor_session(
        &self,
        runtime: &mut RotorSessionRuntime,
        current_track: Option<&Track>,
    ) -> Result<Vec<Track>, ClientError> {
        if let Some(track) = current_track {
            runtime.record_track(track);
        }

        let mut options = GetStationTracksOptions::new(runtime.station_id.clone()).settings2(true);
        if let Some(queue) = runtime.current_track_id() {
            options = options.queue(queue.to_string());
        }

        let response = self.get_station_tracks(&options).await?;
        runtime.session.batch_id = response.batch_id.clone();

        Ok(response.sequence.into_iter().map(|item| item.track).collect())
    }

    pub async fn send_rotor_feedback(
        &self,
        runtime: &RotorSessionRuntime,
        feedback: StationFeedback,
    ) -> Result<(), ClientError> {
        let options = GetStationFeedbackOptions::new(runtime.station_id.clone(), feedback)
            .batch_id(runtime.batch_id().to_string());
        self.send_station_feedback(&options).await
    }
}

fn track_seed(track: &Track) -> Option<String> {
    let album_id = track
        .albums
        .first()
        .and_then(|album| album.id.as_ref().map(|id| id.to_string()))?;

    Some(format!("{}:{}", track.id, album_id))
}

fn missing_runtime_context() -> ClientError {
    ClientError::YandexMusicError {
        error: YandexMusicError {
            name: "MissingRotorRuntimeContext".to_string(),
            message: Some(
                "Rotor session does not contain station_id or radio_session_id".to_string(),
            ),
        },
    }
}
