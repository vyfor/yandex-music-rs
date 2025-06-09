pub mod account_settings;
pub mod promo_code;
pub mod status;

use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub now: String,
    pub uid: Option<i32>,
    pub login: Option<String>,
    pub full_name: Option<String>,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub display_name: Option<String>,
    pub service_available: bool,
    pub hosted_user: Option<bool>,
    pub birthday: Option<String>,
    pub region: Option<i32>,
    #[serde(default)]
    pub passport_phones: Vec<PassportPhone>,
    pub registered_at: Option<String>,
    pub has_info_for_app_metrics: Option<bool>,
    pub child: bool,
    pub non_owner_family_member: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PassportPhone {
    pub phone: String,
}
