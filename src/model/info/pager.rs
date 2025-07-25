use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pager {
    pub page: u32,
    pub per_page: u32,
    pub total: u32,
}
