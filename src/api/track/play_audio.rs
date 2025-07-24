use crate::{api::Endpoint, client::request::RequestOptions, YandexMusicClient, UserId};
use reqwest::Method;
use std::borrow::Cow;

#[derive(Default)]
pub struct PlayAudioOptions {
    pub track_id: Option<String>,
    pub album_id: Option<String>,
    pub playlist_id: Option<String>,
    pub play_id: Option<String>,
    pub from: String,
    pub from_cache: Option<bool>,
    pub uid: Option<UserId>,
    pub track_length_seconds: Option<i32>,
    pub total_played_seconds: Option<i32>,
    pub end_position_seconds: Option<i32>,
    pub timestamp: Option<String>,
    pub client_now: Option<String>,
}

impl PlayAudioOptions {
    pub fn new(from: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            ..Default::default()
        }
    }

    pub fn track_id(mut self, track_id: impl Into<String>) -> Self {
        self.track_id = Some(track_id.into());
        self
    }

    pub fn album_id(mut self, album_id: impl Into<String>) -> Self {
        self.album_id = Some(album_id.into());
        self
    }

    pub fn playlist_id(mut self, playlist_id: impl Into<String>) -> Self {
        self.playlist_id = Some(playlist_id.into());
        self
    }

    pub fn play_id(mut self, play_id: impl Into<String>) -> Self {
        self.play_id = Some(play_id.into());
        self
    }

    pub fn from_cache(mut self, from_cache: bool) -> Self {
        self.from_cache = Some(from_cache);
        self
    }

    pub fn uid(mut self, uid: UserId) -> Self {
        self.uid = Some(uid);
        self
    }

    pub fn track_length_seconds(mut self, track_length_seconds: i32) -> Self {
        self.track_length_seconds = Some(track_length_seconds);
        self
    }

    pub fn total_played_seconds(mut self, total_played_seconds: i32) -> Self {
        self.total_played_seconds = Some(total_played_seconds);
        self
    }

    pub fn end_position_seconds(mut self, end_position_seconds: i32) -> Self {
        self.end_position_seconds = Some(end_position_seconds);
        self
    }

    pub fn timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }

    pub fn client_now(mut self, client_now: impl Into<String>) -> Self {
        self.client_now = Some(client_now.into());
        self
    }
}

impl Endpoint for PlayAudioOptions {
    type Options = ();
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        "play-audio".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Send sending the current state of the track being listened to.
    pub async fn play_audio(&self, options: &PlayAudioOptions) -> Result<(), crate::ClientError> {
        self.request::<(), _>(options).await
    }
}
