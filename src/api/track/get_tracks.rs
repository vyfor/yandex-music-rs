use crate::{
    api::{RequestPath, Response},
    model::track::Track,
    YandexMusicClient,
};

pub struct TracksRequest {}

impl RequestPath for TracksRequest {
    fn path(&self) -> String {
        String::from("tracks")
    }
}

impl YandexMusicClient {
    pub async fn get_tracks(
        &self,
        track_ids: &[i32],
        with_positions: bool,
    ) -> Result<Vec<Track>, crate::ClientError> {
        let response: Response = self
            .post_with_form(
                &TracksRequest {}.path(),
                vec![
                    (
                        "track-ids",
                        &track_ids
                            .iter()
                            .map(|&id| id.to_string() + ",")
                            .collect::<String>(),
                    ),
                    ("with-positions", &with_positions.to_string()),
                ],
            )
            .await?;

        Ok(serde_json::from_value::<Vec<Track>>(response.result)?)
    }
}
