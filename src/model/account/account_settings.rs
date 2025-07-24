use serde::{Deserialize, Serialize};
use crate::UserId;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSettings {
    pub uid: UserId,
    pub last_fm_scrobbling_enabled: bool,
    pub shuffle_enabled: bool,
    pub volume_percents: i32,
    pub modified: String,
    pub facebook_scrobbling_enabled: bool,
    pub add_new_track_on_playlist_top: bool,
    pub user_music_visibility: String,
    pub user_social_visibility: String,
    pub rbt_disabled: bool,
    pub theme: String,
    pub promos_disabled: bool,
    pub auto_play_radio: bool,
    pub sync_queue_enabled: bool,
    pub ads_disabled: Option<bool>,
    pub disk_enabled: Option<bool>,
    pub show_disk_tracks_in_library: Option<bool>,
    pub explicit_forbidden: Option<bool>,
    pub child_mod_enabled: Option<bool>,
    pub wizard_is_passed: Option<bool>,
    pub user_collection_hue: Option<i32>,
}
