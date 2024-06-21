use crate::{api::RequestPath, YandexMusicClient};

pub struct DeletePlaylistRequest {
    pub user_id: i32,
    pub kind: i32,
}

impl RequestPath for DeletePlaylistRequest {
    fn path(&self) -> String {
        format!("users/{}/playlists/{}/delete", self.user_id, self.kind)
    }
}

impl DeletePlaylistRequest {
    pub fn new(user_id: i32, kind: i32) -> Self {
        Self { user_id, kind }
    }
}

impl YandexMusicClient {
    pub async fn delete_playlist(
        &self,
        user_id: i32,
        kind: i32,
    ) -> Result<(), crate::ClientError> {
        self.post(&DeletePlaylistRequest::new(user_id, kind).path())
            .await
    }
}
