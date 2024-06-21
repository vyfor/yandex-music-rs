use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub in_app_products: Vec<String>,
    pub native_products: Vec<String>,
    pub web_payment_url: String,
    pub web_payment_month_product_price: Option<Price>,
    pub promo_codes_enabled: bool,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub product_id: String,
    pub offers_position_id: Option<String>,
    pub item_type: String,
    pub duration: i32,
    pub trial_duration: i32,
    pub feature: String,
    pub debug: bool,
    pub plus: bool,
    pub price: Price,
    pub common_period_duration: Option<String>,
    pub cheapest: Option<bool>,
    pub title: Option<String>,
    pub family_sub: Option<bool>,
    pub fb_image: Option<String>,
    pub fb_name: Option<String>,
    pub family: Option<bool>,
    #[serde(default)]
    pub features: Vec<String>,
    pub description: Option<String>,
    pub available: Option<bool>,
    pub trial_available: Option<bool>,
    pub trial_period_duration: Option<String>,
    pub intro_period_duration: Option<String>,
    pub intro_price: Option<Price>,
    pub start_period_duration: Option<String>,
    pub start_price: Option<Price>,
    pub vendor_trial_available: Option<bool>,
    pub button_text: Option<String>,
    pub button_additional_text: Option<String>,
    #[serde(default)]
    pub payment_method_types: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub amount: i32,
    pub currency: String,
}
