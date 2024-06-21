use crate::{
    api::{RequestPath, Response},
    model::playlist::Playlist,
    YandexMusicClient,
};

pub struct ChangePlaylistVisibilityRequest {
    pub user_id: i32,
    pub kind: i32,
}

impl ChangePlaylistVisibilityRequest {
    pub fn new(user_id: i32, kind: i32) -> Self {
        Self { user_id, kind }
    }
}

impl RequestPath for ChangePlaylistVisibilityRequest {
    fn path(&self) -> String {
        format!("users/{}/playlists/{}/visibility", self.user_id, self.kind)
    }
}

impl YandexMusicClient {
    pub async fn change_playlist_visibility(
        &self,
        user_id: i32,
        kind: i32,
        value: &str,
    ) -> Result<Playlist, crate::ClientError> {
        let response: Response = self
            .post_with_form_str(
                &ChangePlaylistVisibilityRequest::new(user_id, kind).path(),
                vec![("value", value)],
            )
            .await?;

        Ok(serde_json::from_value::<Playlist>(response.result)?)
    }
}
