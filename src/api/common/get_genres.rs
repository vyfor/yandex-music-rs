use crate::{
    api::{RequestPath, Response},
    model::genre::Genre,
    YandexMusicClient,
};

pub struct GenresRequest {}

impl RequestPath for GenresRequest {
    fn path(&self) -> String {
        String::from("genres")
    }
}

impl YandexMusicClient {
    pub async fn get_genres(&self) -> Result<Vec<Genre>, crate::ClientError> {
        let response: Response = self.get(&GenresRequest {}.path()).await?;

        Ok(serde_json::from_value::<Vec<Genre>>(response.result)?)
    }
}
