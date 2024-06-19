use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationIcon {
    pub background_color: String,
    pub image_url: String,
}
