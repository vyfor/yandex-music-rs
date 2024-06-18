use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cover {
    pub r#type: String,
    pub uri: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub custom: bool,
    #[serde(default)]
    pub prefix: Option<String>,
    #[serde(default)]
    pub dir: Option<String>,
}
