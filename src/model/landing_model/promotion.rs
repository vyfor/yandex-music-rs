use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Promotion {
    pub promo_id: String,
    pub title: String,
    pub subtitle: String,
    pub heading: String,
    pub url: String,
    pub url_scheme: String,
    pub text_color: String,
    pub gradient: String,
    pub image: String,
}
