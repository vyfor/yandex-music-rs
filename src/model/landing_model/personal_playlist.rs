use serde::Deserialize;

use crate::model::playlist::Playlist;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalPlaylist {
    pub r#type: String,
    pub ready: bool,
    pub notify: bool,
    pub data: Playlist,
}
