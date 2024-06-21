use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cover {
    pub item_type: Option<String>,
    pub uri: Option<String>,
    #[serde(default)]
    pub items_uri: Vec<String>,
    pub dir: Option<String>,
    pub version: Option<String>,
    pub is_custom: Option<bool>,
    pub custom: Option<bool>,
    pub prefix: Option<String>,
    pub copyright_name: Option<String>,
    pub copyright_cline: Option<String>,
    pub error: Option<String>,
}
