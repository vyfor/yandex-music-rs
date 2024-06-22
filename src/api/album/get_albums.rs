use crate::{
    api::RequestPath,
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
    /// Get a list of albums.
    ///
    /// ### Arguments
    /// * `album_ids` - An array of IDs of the albums.
    ///
    /// ### Returns
    /// * [`Vec<Album>`] - A list of albums.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_albums(
        &self,
        album_ids: &[i32],
    ) -> Result<Vec<Album>, crate::ClientError> {
        let response = self
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

        Ok(serde_json::from_value::<Vec<Album>>(response)?)
    }
}
