use crate::{
    api::RequestPath,
    model::album_model::album::Album,
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
    /// Get album.
    ///
    /// ### Arguments
    /// * `album_id` - The ID of the album.
    ///
    /// ### Returns
    /// * [Album] - The album.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_album(
        &self,
        album_id: i32,
    ) -> Result<Album, crate::ClientError> {
        let response =
            self.get(&AlbumRequest::new(album_id).path()).await?;

        Ok(serde_json::from_value::<Album>(response)?)
    }
}
