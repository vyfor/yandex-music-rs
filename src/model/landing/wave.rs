use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wave {
    pub id: String,
    pub title: String,
    pub items: Vec<WaveItem>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaveItem {
    pub title: String,
    pub header: String,
    pub animation_url: String,
    pub background_image_url: String,
    pub compact_image_url: String,
    pub station_id: String,
    pub seeds: Vec<String>,
    pub colors: WaveColors,
    pub agent: WaveAgent,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaveColors {
    pub average: String,
    pub wave_text: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaveAgent {
    pub animation_uri: String,
    pub cover: AgentCover,
    pub entity: Option<AgentEntity>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentCover {
    pub uri: String,
    pub color: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentEntity {
    #[serde(rename = "type")]
    pub item_type: String,
}
