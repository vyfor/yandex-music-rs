use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cover {
    pub custom: bool,
    pub dir: String,
    pub r#type: String,
    pub uri: String,
    pub version: String,
}
