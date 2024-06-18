use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    #[serde(deserialize_with = "crate::model::utils::string_to_i32")]
    pub id: i32,
    pub value: String,
}
