use serde::Deserialize;

use super::{artist::Artist, label::Label};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: i32,
    pub error: String,
    pub title: String,
    pub r#type: String,
    pub meta_type: String,
    pub year: i32,
    pub release_date: String,
    pub cover_uri: String,
    pub og_image: String,
    pub genre: String,
    pub track_count: i32,
    pub recent: bool,
    pub very_important: bool,
    pub artists: Vec<Artist>,
    pub labels: Vec<Label>,
    pub available: bool,
    pub available_for_premium_users: bool,
    pub available_for_mobile: bool,
    pub available_partially: bool,
    pub bests: Vec<i32>,
}
