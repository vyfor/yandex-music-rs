use serde::Deserialize;

use crate::model::landing::LandingBlock;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Catalogue {
    pub title: String,
    pub blocks: Vec<LandingBlock>,
}
