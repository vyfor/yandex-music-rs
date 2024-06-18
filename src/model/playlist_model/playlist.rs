use serde::Deserialize;

use crate::model::{
    playlist_model::tag::Tag, track_model::cover::Cover, user_model::user::User,
};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub playlist_uuid: String,
    pub description: Option<String>,
    pub description_formatted: Option<String>,
    pub available: bool,
    pub collective: bool,
    pub cover: Cover,
    pub created: String,
    pub modified: String,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub duration_ms: i32,
    pub is_banner: bool,
    pub is_premiere: bool,
    pub kind: i32,
    pub og_image: String,
    pub owner: User,
    pub revision: i32,
    pub snapshot: i32,
    pub tags: Vec<Tag>,
    pub title: String,
    pub track_count: i32,
    pub uid: i32,
    pub visibility: String,
    #[serde(default)]
    pub likes_count: i32,
}
