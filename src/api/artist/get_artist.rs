use crate::{
    api::RequestPath,
    model::artist_model::artist::ArtistInfo,
    YandexMusicClient,
};

pub struct ArtistRequest {
    pub artist_id: i32,
}

impl ArtistRequest {
    pub fn new(artist_id: i32) -> Self {
        Self { artist_id }
    }
}

impl RequestPath for ArtistRequest {
    fn path(&self) -> String {
        format!("artists/{}", self.artist_id)
    }
}

impl YandexMusicClient {
    /// Get artist info.
    ///
    /// ### Arguments
    /// * `artist_id` - The ID of the artist.
    ///
    /// ### Returns
    /// * [ArtistInfo] - The artist info.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_artist(
        &self,
        artist_id: i32,
    ) -> Result<ArtistInfo, crate::ClientError> {
        let response =
            self.get(&ArtistRequest::new(artist_id).path()).await?;

        Ok(serde_json::from_value::<ArtistInfo>(response)?)
    }
}
