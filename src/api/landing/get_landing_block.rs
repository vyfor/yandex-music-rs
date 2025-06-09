use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint,
    client::request::RequestOptions,
    model::landing::landing_item::{LandingBlockType, LandingItem},
    YandexMusicClient,
};

/// Request for getting a specific landing block.
pub struct GetLandingBlockOptions {
    /// The type of landing block to retrieve.
    pub block: LandingBlockType,
}

impl GetLandingBlockOptions {
    /// Create a new request for getting a specific landing block.
    pub fn new(block: LandingBlockType) -> Self {
        Self { block }
    }
}

impl Endpoint for GetLandingBlockOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        match &self.block {
            LandingBlockType::Chart(Some(chart_type)) => {
                format!("landing3/chart/{chart_type}").into()
            }
            _ => format!("landing3/{}", self.block).into(),
        }
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve a specific landing block by its type.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the block type to retrieve.
    ///
    /// ### Returns
    /// * `Result<LandingItem, ClientError>` - The landing block data or an error if the request fails.
    pub async fn get_landing_block(
        &self,
        options: &GetLandingBlockOptions,
    ) -> Result<LandingItem, crate::ClientError> {
        self.request::<LandingItem, _>(options).await
    }
}
