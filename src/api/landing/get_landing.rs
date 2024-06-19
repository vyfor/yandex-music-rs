use crate::{
    api::{RequestPath, Response},
    model::landing::{Landing, LandingType},
    YandexMusicClient,
};

pub struct LandingRequest {
    blocks: Vec<LandingType>,
}

impl LandingRequest {
    pub fn new(blocks: Vec<LandingType>) -> Self {
        Self { blocks }
    }
}

impl RequestPath for LandingRequest {
    fn path(&self) -> String {
        format!(
            "landing3?blocks={}",
            self.blocks
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<_>>()
                .join(","),
        )
    }
}

impl YandexMusicClient {
    pub async fn get_landing(
        &self,
        blocks: Vec<LandingType>,
    ) -> Result<Landing, crate::ClientError> {
        let response: Response =
            self.get(&LandingRequest::new(blocks).path()).await?;

        Ok(serde_json::from_value::<Landing>(response.result)?)
    }
}
