use crate::{
    api::{RequestPath, Response},
    model::playlist::{ModifyPlaylistDiff, Playlist},
    YandexMusicClient,
};

pub struct ModifyPlaylistRequest {
    pub user_id: i32,
    pub kind: i32,
}

impl RequestPath for ModifyPlaylistRequest {
    fn path(&self) -> String {
        format!(
            "users/{}/playlists/{}/change-relative",
            self.user_id, self.kind
        )
    }
}

impl ModifyPlaylistRequest {
    pub fn new(user_id: i32, kind: i32) -> Self {
        Self { user_id, kind }
    }
}

impl YandexMusicClient {
    pub async fn modify_playlist(
        &self,
        user_id: i32,
        kind: i32,
        diff: ModifyPlaylistDiff,
        revision: i32,
    ) -> Result<Playlist, crate::ClientError> {
        let response: Response = self
            .post_with_form_str(
                &ModifyPlaylistRequest::new(user_id, kind).path(),
                vec![
                    ("diff", &serde_json::to_string(&diff)?),
                    ("revision", &revision.to_string()),
                ],
            )
            .await?;

        Ok(serde_json::from_value::<Playlist>(response.result)?)
    }
}
