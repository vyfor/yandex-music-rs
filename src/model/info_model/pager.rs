use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pager {
    pub page: i32,
    pub per_page: i32,
    pub total: i32,
}
