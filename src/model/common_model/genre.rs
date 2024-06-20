use std::collections::HashMap;

use serde::Deserialize;

use crate::model::icon::Icon;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    pub id: String,
    pub weight: i32,
    pub composer_top: bool,
    pub title: String,
    pub titles: HashMap<String, GenreTitle>,
    pub images: GenreImages,
    pub show_in_menu: bool,
    #[serde(default)]
    pub show_in_regions: Vec<i32>,
    pub full_title: Option<String>,
    pub url_part: Option<String>,
    pub color: Option<String>,
    pub radio_icon: Option<Icon>,
    pub sub_genres: Vec<Genre>,
    #[serde(default)]
    pub hide_in_regions: Vec<i32>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenreTitle {
    pub title: String,
    pub full_title: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenreImages {
    #[serde(rename = "208x208")]
    pub _208x208: String,
    #[serde(rename = "300x300")]
    pub _300x300: String,
}
