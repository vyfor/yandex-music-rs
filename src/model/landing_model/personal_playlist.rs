use serde::Deserialize;

use crate::model::playlist_model::playlist::Playlist;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalPlaylist {
    #[serde(rename="type")]
    pub item_type: String,
    pub ready: bool,
    pub notify: bool,
    pub data: Option<Playlist>,
}
