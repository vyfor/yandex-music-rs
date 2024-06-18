use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomWave {
    pub title: String,
    pub header: String,
    pub animation_url: Option<String>,
    pub background_image_url: Option<String>,
}
