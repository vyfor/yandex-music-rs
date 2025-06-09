use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalPlaylists {
    pub is_wizard_passed: bool,
}
