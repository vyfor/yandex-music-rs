use crate::{
    api::RequestPath,
    model::common_model::catalogue::Catalogue,
    YandexMusicClient,
};

pub struct NonMusicCatalogueRequest {}

impl RequestPath for NonMusicCatalogueRequest {
    fn path(&self) -> String {
        String::from("non-music/catalogue")
    }
}

impl YandexMusicClient {
    /// Get non-music catalogue.
    ///
    /// ### Returns
    /// * [Catalogue] - The non-music catalogue.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_non_music_catalogue(
        &self,
    ) -> Result<Catalogue, crate::ClientError> {
        let response =
            self.get(&NonMusicCatalogueRequest {}.path()).await?;

        Ok(serde_json::from_value::<Catalogue>(response)?)
    }
}
