pub mod best_result;
pub mod search_result;
pub mod search_result_item;
pub mod suggestion;

use std::fmt::Display;

use serde::Deserialize;

use best_result::BestResult;
use search_result::SearchResult;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    pub search_request_id: String,
    pub text: String,
    pub best: Option<BestResult>,
    pub albums: Option<SearchResult>,
    pub artists: Option<SearchResult>,
    pub playlists: Option<SearchResult>,
    pub tracks: Option<SearchResult>,
    pub users: Option<SearchResult>,
    pub videos: Option<SearchResult>,
    pub podcasts: Option<SearchResult>,
    pub podcast_episodes: Option<SearchResult>,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub misspell_result: Option<String>,
    pub misspell_original: Option<String>,
    pub misspell_corrected: Option<bool>,
    pub nocorrect: Option<bool>,
}

pub enum SearchType {
    Artists,
    Albums,
    Tracks,
    Podcasts,
    All,
}

impl Display for SearchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchType::Artists => write!(f, "artist"),
            SearchType::Albums => write!(f, "album"),
            SearchType::Tracks => write!(f, "track"),
            SearchType::Podcasts => write!(f, "podcast"),
            SearchType::All => write!(f, "all"),
        }
    }
}
