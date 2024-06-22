use serde::Deserialize;

use super::station::Station;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rotor {
    pub station: Station,
    pub settings: RotorSettings,
    pub settings2: RotorSettings,
    pub ad_params: RotorAdParams,
    pub rup_title: String,
    pub rup_description: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RotorSettings {
    pub language: String,
    pub diversity: String,
    pub mood: Option<i32>,
    pub mood_energy: Option<String>,
    pub energy: Option<i32>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RotorAdParams {
    #[serde(deserialize_with = "crate::model::utils::string_to_i32")]
    pub partner_id: i32,
    #[serde(deserialize_with = "crate::model::utils::string_to_i32")]
    pub category_id: i32,
    pub page_ref: String,
    pub target_ref: String,
    pub other_params: String,
    pub ad_volume: i32,
    pub genre_id: Option<i32>,
    pub genre_name: Option<String>,
}
