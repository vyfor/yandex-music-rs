use serde::Deserialize;

pub type UserId = u64;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uid: UserId,
    pub login: String,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub full_name: Option<String>,
    pub sex: Option<String>,
    pub verified: Option<bool>,
    #[serde(default)]
    pub regions: Vec<i32>,
}
