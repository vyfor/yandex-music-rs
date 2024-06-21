use crate::{
    api::{RequestPath, Response},
    model::playlist::Playlist,
    YandexMusicClient,
};

pub struct CreatePlaylistRequest {
    pub user_id: i32,
}

impl CreatePlaylistRequest {
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
    }
}

impl RequestPath for CreatePlaylistRequest {
    fn path(&self) -> String {
        format!("users/{}/playlists/create", self.user_id)
    }
}

impl YandexMusicClient {
    pub async fn create_playlist(
        &self,
        user_id: i32,
        title: &str,
        visibility: &str,
    ) -> Result<Playlist, crate::ClientError> {
        let response: Response = self
            .post_with_form(
                &CreatePlaylistRequest::new(user_id).path(),
                vec![("title", title), ("visibility", visibility)],
            )
            .await?;

        Ok(serde_json::from_value::<Playlist>(response.result)?)
    }
}
