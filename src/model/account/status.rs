use serde::Deserialize;

use super::Account;
use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountStatus {
    pub account: Account,
    pub permissions: AccountPermissions,
    pub advertisement: Option<String>,
    pub cache_limit: Option<i32>,
    pub subeditor: Option<bool>,
    pub subeditor_level: Option<i32>,
    pub plus: AccountPlus,
    pub default_email: Option<String>,
    pub skips_per_hour: Option<u32>,
    pub station_exists: Option<bool>,
    pub station_data: Option<AccountStationData>,
    pub experiment: Option<i32>,
    pub pretrial_active: Option<bool>,
    pub userhash: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPermissions {
    pub until: DateTime<Utc>,
    pub values: Vec<String>,
    pub default: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPlus {
    pub has_plus: bool,
    pub is_tutorial_completed: bool,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountStationData {
    pub name: String,
}
