use crate::{
    api::RequestPath,
    model::artist_model::artist::{ArtistAlbums, SortBy},
    YandexMusicClient,
};

pub struct ArtistAlbumsRequest {
    pub artist_id: i32,
    pub page: u32,
    pub page_size: u32,
    pub sort_by: SortBy,
}

impl ArtistAlbumsRequest {
    pub fn new(artist_id: i32, sort_by: SortBy) -> Self {
        Self {
            artist_id,
            page: 0,
            page_size: 20,
            sort_by,
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

impl RequestPath for ArtistAlbumsRequest {
    fn path(&self) -> String {
        format!(
            "artists/{}/direct-albums?page={}&page-size={}&sort-by={}",
            self.artist_id, self.page, self.page_size, self.sort_by
        )
    }
}

impl YandexMusicClient {
    /// Get artist albums.
    ///
    /// ### Arguments
    /// * `artist_id` - The ID of the artist.
    /// * `sort_by` - The order of sorting albums.
    ///
    /// ### Returns
    /// * [ArtistAlbums] - The artist albums.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_artist_albums(
        &self,
        artist_id: i32,
        sort_by: SortBy,
    ) -> Result<ArtistAlbums, crate::ClientError> {
        let response = self
            .get(&ArtistAlbumsRequest::new(artist_id, sort_by).path())
            .await?;

        Ok(serde_json::from_value::<ArtistAlbums>(response)?)
    }

    /// Get artist albums with pagination.
    ///
    /// ### Arguments
    /// * `artist_id` - The ID of the artist.
    /// * `sort_by` - The order of sorting albums.
    /// * `page` - The zero-indexed page number.
    /// * `page_size` - The page size.
    ///
    /// ### Returns
    /// * [ArtistAlbums] - The artist albums.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_artist_albums_with_page(
        &self,
        artist_id: i32,
        sort_by: SortBy,
        page: u32,
        page_size: u32,
    ) -> Result<ArtistAlbums, crate::ClientError> {
        let response = self
            .get(
                &ArtistAlbumsRequest::new(artist_id, sort_by)
                    .with_page(page)
                    .with_page_size(page_size)
                    .path(),
            )
            .await?;

        Ok(serde_json::from_value::<ArtistAlbums>(response)?)
    }
}
