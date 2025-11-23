use crate::{
    api::Endpoint, client::request::RequestOptions, error::ClientError,
    model::rotor::session::Session, YandexMusicClient,
};

#[derive(Default)]
pub struct GetSessionTracksOptions {
    /// The ID of the session to retrieve tracks for.
    pub session_id: String,
    /// The queue of track identifiers in the format of "{track_id}:{album_id}".
    pub queue: Vec<String>,
}

impl GetSessionTracksOptions {
    pub fn new<S, I>(session_id: S, queue: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        Self {
            session_id: session_id.into(),
            queue: queue.into_iter().map(Into::into).collect(),
        }
    }
}

impl Endpoint for GetSessionTracksOptions {
    type Options = ();
    const METHOD: reqwest::Method = reqwest::Method::POST;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        format!("rotor/session/{}/tracks", self.session_id).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default().with_json_data(serde_json::json!({
            "queue": self.queue,
        }))
    }
}

impl YandexMusicClient {
    /// Retrieve tracks for a specific rotor session.
    ///
    /// ### Arguments
    ///
    /// * `options` - Options for retrieving session tracks, including session ID and queue.
    ///
    /// ### Returns
    /// * `[Session]` - A result containing the session tracks response or an error.
    pub async fn get_session_tracks(
        &self,
        options: GetSessionTracksOptions,
    ) -> Result<Session, ClientError> {
        self.request(&options).await
    }
}
