use serde::Deserialize;

use crate::model::playlist::Playlist;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalPlaylist {
    // todo: rename all `r#type`s to `item_type`
    pub r#type: String,
    pub ready: bool,
    pub notify: bool,
    pub data: Option<Playlist>,
}
