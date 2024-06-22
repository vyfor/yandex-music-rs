use crate::{
    api::RequestPath,
    model::common_model::genre::Genre,
    YandexMusicClient,
};

pub struct GenresRequest {}

impl RequestPath for GenresRequest {
    fn path(&self) -> String {
        String::from("genres")
    }
}

impl YandexMusicClient {
    /// Get genres.
    ///
    /// ### Returns
    /// * [`Vec<Genre>`] - The list of genres.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_genres(&self) -> Result<Vec<Genre>, crate::ClientError> {
        let response = self.get(&GenresRequest {}.path()).await?;

        Ok(serde_json::from_value::<Vec<Genre>>(response)?)
    }
}
