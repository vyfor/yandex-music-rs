pub mod dashboard;
pub mod feedback;
pub mod id;
pub mod restrictions;
pub mod station;

use serde::Deserialize;

use station::Station;

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
    pub mood: Option<u8>,
    pub mood_energy: Option<String>,
    pub energy: Option<u8>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RotorAdParams {
    pub partner_id: String,
    pub category_id: String,
    pub page_ref: String,
    pub target_ref: String,
    pub other_params: String,
    pub ad_volume: u8,
    pub genre_id: Option<u32>,
    pub genre_name: Option<String>,
}
