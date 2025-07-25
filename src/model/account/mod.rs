pub mod promo_code;
pub mod settings;
pub mod status;

use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub now: DateTime<Utc>,
    pub uid: Option<u64>,
    pub login: Option<String>,
    pub full_name: Option<String>,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub display_name: Option<String>,
    pub service_available: bool,
    pub hosted_user: Option<bool>,
    pub birthday: Option<NaiveDate>,
    pub region: Option<u32>,
    #[serde(default)]
    pub passport_phones: Vec<PassportPhone>,
    pub registered_at: Option<DateTime<Utc>>,
    pub has_info_for_app_metrics: Option<bool>,
    pub child: bool,
    pub non_owner_family_member: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PassportPhone {
    pub phone: String,
}
