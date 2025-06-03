use crate::{
    api::RequestPath, model::playlist_model::playlist::Playlist,
    YandexMusicClient,
};

pub struct PlaylistsRequest {
    pub user_id: i32,
    pub kinds: Vec<i32>,
    pub mixed: bool,
    pub with_tracks: bool,
    pub rich_tracks: bool,
}

impl PlaylistsRequest {
    pub fn new(user_id: i32) -> Self {
        Self {
            user_id,
            kinds: Vec::new(),
            mixed: false,
            with_tracks: false,
            rich_tracks: false,
        }
    }

    pub fn kinds(mut self, kinds: Vec<i32>) -> Self {
        self.kinds = kinds;
        self
    }

    pub fn mixed(mut self, mixed: bool) -> Self {
        self.mixed = mixed;
        self
    }

    pub fn with_tracks(mut self, with_tracks: bool) -> Self {
        self.with_tracks = with_tracks;
        self
    }

    pub fn rich_tracks(mut self, rich_tracks: bool) -> Self {
        self.rich_tracks = rich_tracks;
        self
    }
}

impl RequestPath for PlaylistsRequest {
    fn path(&self) -> String {
        format!("users/{}/playlists", self.user_id)
    }
}

impl YandexMusicClient {
    /// Get playlists.
    ///
    /// ### Arguments
    /// * `request` - The request parameters.
    ///
    /// ### Returns
    /// * [`Vec<Playlist>`] - A list of playlists.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_playlists(
        &self,
        request: &PlaylistsRequest,
    ) -> Result<Vec<Playlist>, crate::ClientError> {
        let response = self
            .get(&format!(
                "{}?kinds={}&mixed={}&withTracks={}&richTracks={}",
                request.path(),
                request
                    .kinds
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
                request.mixed,
                request.with_tracks,
                request.rich_tracks
            ))
            .await?;

        Ok(serde_json::from_value::<Vec<Playlist>>(response)?)
    }
}
