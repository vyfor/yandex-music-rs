/*
    url = session/new
    payload = {"seeds":["user:onyourwave"],"includeTracksInResponse":true,"includeWaveModel":true,"interactive":true}
    method = POST
    response =
*/

use crate::{
    api::Endpoint, client::request::RequestOptions, error::ClientError,
    model::rotor::session::Session, YandexMusicClient,
};

#[derive(Default)]
pub struct CreateSessionOptions {
    /// The seed identifiers to create the session from.
    pub seeds: Vec<String>,
    /// The optional queue of track identifiers in the format of "{track_id}:{album_id}".
    pub queue: Vec<String>,
    /// Whether to include tracks in the response.
    pub include_tracks_in_response: bool,
    /// Whether to include the wave model in the response.
    pub include_wave_model: bool,
    /// Whether the session is interactive.
    pub interactive: bool,
}

impl CreateSessionOptions {
    pub fn new<S, I>(seeds: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        Self {
            seeds: seeds.into_iter().map(Into::into).collect(),
            queue: Vec::new(),
            include_tracks_in_response: true,
            include_wave_model: true,
            interactive: false,
        }
    }

    pub fn queue<S, I>(mut self, queue: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        self.queue = queue.into_iter().map(Into::into).collect();
        self
    }

    pub fn include_tracks_in_response(mut self, include: bool) -> Self {
        self.include_tracks_in_response = include;
        self
    }

    pub fn include_wave_model(mut self, include: bool) -> Self {
        self.include_wave_model = include;
        self
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }
}

impl Endpoint for CreateSessionOptions {
    type Options = ();
    const METHOD: reqwest::Method = reqwest::Method::POST;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        "rotor/session/new".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default().with_json_data(serde_json::json!({
            "seeds": self.seeds,
            "queue": self.queue,
            "includeTracksInResponse": self.include_tracks_in_response,
            "includeWaveModel": self.include_wave_model,
            "interactive": self.interactive,
        }))
    }
}

impl YandexMusicClient {
    /// Create a new rotor session.
    ///
    /// ### Arguments
    ///
    /// * `options` - Options for creating a new session, including seeds and queue.
    ///
    /// ### Returns
    /// * `[Session]` - A result containing the created session or an error.
    pub async fn create_session(
        &self,
        options: CreateSessionOptions,
    ) -> Result<Session, ClientError> {
        self.request(&options).await
    }
}
