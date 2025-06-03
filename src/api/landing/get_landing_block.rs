use crate::{
    api::RequestPath,
    model::landing_model::landing_item::{LandingBlockType, LandingItem},
    YandexMusicClient,
};

pub struct LandingBlockRequest {
    block: LandingBlockType,
}

impl LandingBlockRequest {
    pub fn new(block: LandingBlockType) -> Self {
        Self { block }
    }
}

impl RequestPath for LandingBlockRequest {
    fn path(&self) -> String {
        match &self.block {
            LandingBlockType::Chart(chart_type) => {
                if let Some(chart_type) = chart_type {
                    format!("landing3/chart/{chart_type}")
                } else {
                    "landing3/chart".to_string()
                }
            }
            _ => format!("landing3/{}", self.block),
        }
    }
}

impl YandexMusicClient {
    /// Get landing block.
    ///
    /// ### Arguments
    /// * `block` - The block type.
    ///
    /// ### Returns
    /// * [LandingItem] - The landing block item.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_landing_block(
        &self,
        block: LandingBlockType,
    ) -> Result<LandingItem, crate::ClientError> {
        let response =
            self.get(&LandingBlockRequest::new(block).path()).await?;

        Ok(serde_json::from_value::<LandingItem>(response)?)
    }
}
