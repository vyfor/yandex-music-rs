use crate::{
    api::{RequestPath, Response},
    model::catalogue::Catalogue,
    YandexMusicClient,
};

pub struct NonMusicCatalogueRequest {}

impl RequestPath for NonMusicCatalogueRequest {
    fn path(&self) -> String {
        String::from("non-music/catalogue")
    }
}

impl YandexMusicClient {
    pub async fn get_non_music_catalogue(
        &self,
    ) -> Result<Catalogue, crate::ClientError> {
        let response: Response =
            self.get(&NonMusicCatalogueRequest {}.path()).await?;

        Ok(serde_json::from_value::<Catalogue>(response.result)?)
    }
}
