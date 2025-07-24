use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::playlist::Playlist, YandexMusicClient,
    UserId
};

/// Request for getting user's playlists.
pub struct GetPlaylistsOptions {
    /// The ID of the user whose playlists to retrieve.
    pub user_id: UserId,
    /// Specific playlist kinds to include. If empty, all playlists are returned.
    pub kinds: Vec<i32>,
    /// Whether to include mixed content in the response.
    pub mixed: bool,
    /// Whether to include tracks in the response.
    pub with_tracks: bool,
    /// Whether to include rich track information.
    pub rich_tracks: bool,
}

impl GetPlaylistsOptions {
    /// Create a new request for getting user's playlists.
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            kinds: Vec::new(),
            mixed: false,
            with_tracks: false,
            rich_tracks: false,
        }
    }

    /// Set specific playlist kinds to include.
    pub fn kinds<I>(mut self, kinds: I) -> Self
    where
        I: IntoIterator<Item = i32>,
    {
        self.kinds = kinds.into_iter().collect();
        self
    }

    /// Set whether to include mixed content.
    pub fn mixed(mut self, mixed: bool) -> Self {
        self.mixed = mixed;
        self
    }

    /// Set whether to include tracks in the response.
    pub fn with_tracks(mut self, with_tracks: bool) -> Self {
        self.with_tracks = with_tracks;
        self
    }

    /// Set whether to include rich track information.
    pub fn rich_tracks(mut self, rich_tracks: bool) -> Self {
        self.rich_tracks = rich_tracks;
        self
    }
}

impl Endpoint for GetPlaylistsOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        let kinds = self
            .kinds
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");

        format!(
            "users/{}/playlists?kinds={}&mixed={}&withTracks={}&richTracks={}",
            self.user_id, kinds, self.mixed, self.with_tracks, self.rich_tracks
        )
        .into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve a list of playlists for a specific user.
    ///
    /// ### Arguments
    /// * `options` - The request options containing user ID and query parameters.
    ///
    /// ### Returns
    /// * `Result<Vec<Playlist>, ClientError>` - A list of playlists or an error if the request fails.
    pub async fn get_playlists(
        &self,
        options: &GetPlaylistsOptions,
    ) -> Result<Vec<Playlist>, crate::ClientError> {
        self.request::<Vec<Playlist>, _>(options).await
    }
}
