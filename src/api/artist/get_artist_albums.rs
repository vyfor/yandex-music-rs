use crate::{
    api::{RequestPath, Response},
    model::artist_model::artist::{ArtistAlbums, SortBy},
    YandexMusicClient,
};

pub struct ArtistAlbumsRequest {
    pub artist_id: i32,
    pub page: u32,
    pub page_size: u32,
    pub sort_by: Option<SortBy>,
}

impl ArtistAlbumsRequest {
    pub fn new(artist_id: i32) -> Self {
        Self {
            artist_id,
            page: 0,
            page_size: 20,
            sort_by: None,
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

    pub fn with_sort_by(mut self, sort_by: SortBy) -> Self {
        self.sort_by = Some(sort_by);
        self
    }
}

impl RequestPath for ArtistAlbumsRequest {
    fn path(&self) -> String {
        format!(
            "artists/{}/direct-albums?page={}&page-size={}&sort-by={}",
            self.artist_id,
            self.page,
            self.page_size,
            self.sort_by
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_default()
        )
    }
}

impl YandexMusicClient {
    pub async fn get_artist_albums(
        &self,
        artist_id: i32,
    ) -> Result<ArtistAlbums, crate::ClientError> {
        let response: Response = self
            .get(&ArtistAlbumsRequest::new(artist_id).path())
            .await?;

        Ok(serde_json::from_value::<ArtistAlbums>(response.result)?)
    }

    pub async fn get_artist_albums_with_page(
        &self,
        artist_id: i32,
        page: u32,
        page_size: u32,
        sort_by: SortBy,
    ) -> Result<ArtistAlbums, crate::ClientError> {
        let response: Response = self
            .get(
                &ArtistAlbumsRequest::new(artist_id)
                    .with_page(page)
                    .with_page_size(page_size)
                    .with_sort_by(sort_by)
                    .path(),
            )
            .await?;

        Ok(serde_json::from_value::<ArtistAlbums>(response.result)?)
    }
}
