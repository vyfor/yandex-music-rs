use crate::{
    api::{RequestPath, Response},
    model::album_model::album::Album,
    YandexMusicClient,
};

pub struct AlbumsRequest {}

impl RequestPath for AlbumsRequest {
    fn path(&self) -> String {
        String::from("albums")
    }
}

impl YandexMusicClient {
    pub async fn get_albums(
        &self,
        album_ids: &[i32],
    ) -> Result<Vec<Album>, crate::ClientError> {
        let response: Response = self
            .post_with_form_str(
                &AlbumsRequest {}.path(),
                vec![(
                    "album-ids",
                    &album_ids
                        .iter()
                        .map(|&id| id.to_string() + ",")
                        .collect::<String>(),
                )],
            )
            .await?;

        Ok(serde_json::from_value::<Vec<Album>>(response.result)?)
    }
}
