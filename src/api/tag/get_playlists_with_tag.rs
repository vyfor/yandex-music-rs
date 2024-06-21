use crate::{
    api::{RequestPath, Response},
    model::info_model::tag::TaggedPlaylistIds,
    YandexMusicClient,
};

pub struct TaggedPlaylistsRequest {
    pub tag: String,
}

impl TaggedPlaylistsRequest {
    pub fn new(tag: String) -> Self {
        Self { tag }
    }
}

impl RequestPath for TaggedPlaylistsRequest {
    fn path(&self) -> String {
        format!("/tags/{}/playlist-ids", self.tag)
    }
}

impl YandexMusicClient {
    pub async fn get_tagged_playlist_ids(
        &self,
        tag: String,
    ) -> Result<TaggedPlaylistIds, crate::ClientError> {
        let response: Response =
            self.get(&TaggedPlaylistsRequest::new(tag).path()).await?;

        Ok(serde_json::from_value::<TaggedPlaylistIds>(
            response.result,
        )?)
    }
}
