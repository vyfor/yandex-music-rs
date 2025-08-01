use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::playlist::Playlist, YandexMusicClient,
};

/// Request for getting a specific playlist.
pub struct GetPlaylistOptions {
    /// The ID of the user who owns the playlist.
    pub user_id: u64,
    /// The kind (ID) of the playlist.
    pub kind: u32,
    /// The page number for pagination.
    pub page: u32,
    /// The number of tracks per page.
    pub page_size: u32,
    /// Whether to include track play counts.
    pub track_play_counts: bool,
    /// Whether to include rich track information.
    pub rich_tracks: bool,
}

impl GetPlaylistOptions {
    /// Create a new request for getting a specific playlist.
    pub fn new(user_id: u64, kind: u32) -> Self {
        Self {
            user_id,
            kind,
            page: 1,
            page_size: 0,
            track_play_counts: false,
            rich_tracks: true,
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn track_play_counts(mut self, track_play_counts: bool) -> Self {
        self.track_play_counts = track_play_counts;
        self
    }

    pub fn rich_tracks(mut self, rich_tracks: bool) -> Self {
        self.rich_tracks = rich_tracks;
        self
    }
}

impl Endpoint for GetPlaylistOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!(
            "users/{}/playlists/{}?page={}&pageSize={}&trackPlayCounts={}&richTracks={}",
            self.user_id,
            self.kind,
            self.page,
            self.page_size,
            self.track_play_counts,
            self.rich_tracks
        )
        .into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve a specific playlist by user ID and playlist kind.
    ///
    /// ### Arguments
    /// * `options` - The request options.
    ///
    /// ### Returns
    /// * `Result<Playlist, ClientError>` - The playlist data or an error if the request fails.
    pub async fn get_playlist(
        &self,
        options: &GetPlaylistOptions,
    ) -> Result<Playlist, crate::ClientError> {
        self.request(options).await
    }
}
