use serde::Deserialize;

use crate::model::playlist_model::id::PlaylistId;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    #[serde(deserialize_with = "crate::model::utils::string_to_i32")]
    pub id: i32,
    pub value: String,
    pub name: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaggedPlaylistIds {
    pub tag: Tag,
    pub ids: Vec<PlaylistId>,
}
