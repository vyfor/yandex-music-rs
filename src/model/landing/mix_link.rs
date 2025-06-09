use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MixLink {
    pub title: String,
    pub url: String,
    pub url_scheme: String,
    pub text_color: String,
    pub background_color: String,
    pub background_image_url: String,
    pub cover_white: Option<String>,
    pub cover_uri: Option<String>,
}
