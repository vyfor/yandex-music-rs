use crate::{
    api::{RequestPath, Response},
    model::artist::ArtistTrackIds,
    YandexMusicClient,
};

pub struct ArtistTrackIdsRequest {
    pub artist_id: i32,
}

impl ArtistTrackIdsRequest {
    pub fn new(artist_id: i32) -> Self {
        Self { artist_id }
    }
}

impl RequestPath for ArtistTrackIdsRequest {
    fn path(&self) -> String {
        format!("artists/{}/track-ids-by-rating", self.artist_id)
    }
}

impl YandexMusicClient {
    pub async fn get_artist_track_ids(
        &self,
        artist_id: i32,
    ) -> Result<ArtistTrackIds, crate::ClientError> {
        let response: Response = self
            .get(&ArtistTrackIdsRequest::new(artist_id).path())
            .await?;

        Ok(serde_json::from_value::<ArtistTrackIds>(response.result)?)
    }
}
