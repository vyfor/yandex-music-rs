use crate::{
    api::{RequestPath, Response},
    model::artist_model::artist::ArtistTracks,
    YandexMusicClient,
};

pub struct ArtistTracksRequest {
    pub artist_id: i32,
    pub page: u32,
    pub page_size: u32,
}

impl ArtistTracksRequest {
    pub fn new(artist_id: i32) -> Self {
        Self {
            artist_id,
            page: 0,
            page_size: 20,
        }
    }

    pub fn with_page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub fn with_page_size(mut self, page_size: u32) -> Self {
        self.page_size = page_size;
        self
    }
}

impl RequestPath for ArtistTracksRequest {
    fn path(&self) -> String {
        format!(
            "artists/{}/tracks?page={}&page-size={}",
            self.artist_id, self.page, self.page_size
        )
    }
}

impl YandexMusicClient {
    pub async fn get_artist_tracks(
        &self,
        artist_id: i32,
    ) -> Result<ArtistTracks, crate::ClientError> {
        let response: Response = self
            .get(&ArtistTracksRequest::new(artist_id).path())
            .await?;

        Ok(serde_json::from_value::<ArtistTracks>(response.result)?)
    }

    pub async fn get_artist_tracks_with_page(
        &self,
        artist_id: i32,
        page: u32,
        page_size: u32,
    ) -> Result<ArtistTracks, crate::ClientError> {
        let response: Response = self
            .get(
                &ArtistTracksRequest::new(artist_id)
                    .with_page(page)
                    .with_page_size(page_size)
                    .path(),
            )
            .await?;

        Ok(serde_json::from_value::<ArtistTracks>(response.result)?)
    }
}
