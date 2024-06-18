use crate::{
    api::{RequestPath, Response},
    model::album::Album,
    YandexMusicClient,
};

pub struct AlbumRequest {
    pub album_id: i32,
}

impl AlbumRequest {
    pub fn new(album_id: i32) -> Self {
        Self { album_id }
    }
}

impl RequestPath for AlbumRequest {
    fn path(&self) -> String {
        format!("albums/{}", self.album_id)
    }
}

impl YandexMusicClient {
    pub async fn get_album(
        &self,
        album_id: i32,
    ) -> Result<Album, crate::ClientError> {
        let response: Response =
            self.get(&AlbumRequest::new(album_id).path()).await?;

        Ok(serde_json::from_value::<Album>(response.result)?)
    }
}
