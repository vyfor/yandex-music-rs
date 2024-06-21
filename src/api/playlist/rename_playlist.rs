use crate::{
    api::{RequestPath, Response},
    model::playlist::Playlist,
    YandexMusicClient,
};

pub struct RenamePlaylistRequest {
    pub user_id: i32,
    pub kind: i32,
}

impl RenamePlaylistRequest {
    pub fn new(user_id: i32, kind: i32) -> Self {
        Self { user_id, kind }
    }
}

impl RequestPath for RenamePlaylistRequest {
    fn path(&self) -> String {
        format!("users/{}/playlists/{}/name", self.user_id, self.kind)
    }
}

impl YandexMusicClient {
    pub async fn rename_playlist(
        &self,
        user_id: i32,
        kind: i32,
        value: &str,
    ) -> Result<Playlist, crate::ClientError> {
        let response: Response = self
            .post_with_form(
                &RenamePlaylistRequest::new(user_id, kind).path(),
                vec![("value", value)],
            )
            .await?;

        Ok(serde_json::from_value::<Playlist>(response.result)?)
    }
}
