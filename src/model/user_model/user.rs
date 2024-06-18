use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uid: i32,
    pub login: String,
    pub name: String,
    pub sex: String,
    pub verified: bool,
}
