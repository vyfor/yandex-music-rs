use serde::Deserialize;

use super::status::AccountStatus;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromoCode {
    pub status: String,
    pub status_desc: String,
    pub account_status: AccountStatus,
}
