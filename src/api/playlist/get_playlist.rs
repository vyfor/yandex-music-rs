use crate::{
    api::{RequestPath, Response},
    error::ClientError,
    model::playlist::Playlist,
    YandexMusicClient,
};

pub struct PlaylistRequest {
    pub user_id: i32,
    pub kind: i32,
}

impl PlaylistRequest {
    pub fn new(user_id: i32, kind: i32) -> Self {
        Self { user_id, kind }
    }
}

impl RequestPath for PlaylistRequest {
    fn path(&self) -> String {
        format!("/users/{}/playlists/{}", self.user_id, self.kind)
    }
}

impl YandexMusicClient {
    pub async fn get_playlist(
        &self,
        user_id: i32,
        kind: i32,
    ) -> Result<Playlist, ClientError> {
        let response: Response = self
            .get(&PlaylistRequest::new(user_id, kind).path())
            .await?;

        Ok(serde_json::from_value::<Playlist>(response.result)?)
    }
}