use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use super::search_result_item::SearchResultItem;

#[derive(Debug, PartialEq, Clone)]
pub struct BestResult {
    pub item_type: String,
    pub text: Option<String>,
    pub result: SearchResultItem,
}

impl<'de> Deserialize<'de> for BestResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct BestResultHelper {
            #[serde(rename = "type")]
            item_type: String,
            text: Option<String>,
            result: Value,
        }

        let helper = BestResultHelper::deserialize(deserializer)?;

        let result_item = match helper.item_type.as_str() {
            "track" => SearchResultItem::Track(
                serde_json::from_value(helper.result).map_err(Error::custom)?,
            ),
            "artist" => SearchResultItem::Artist(
                serde_json::from_value(helper.result).map_err(Error::custom)?,
            ),
            "album" => SearchResultItem::Album(
                serde_json::from_value(helper.result).map_err(Error::custom)?,
            ),
            "playlist" => SearchResultItem::Playlist(
                serde_json::from_value(helper.result).map_err(Error::custom)?,
            ),
            "video" => SearchResultItem::Video(
                serde_json::from_value(helper.result).map_err(Error::custom)?,
            ),
            "user" => SearchResultItem::User(
                serde_json::from_value(helper.result).map_err(Error::custom)?,
            ),
            "podcast" => SearchResultItem::Podcast(
                serde_json::from_value(helper.result).map_err(Error::custom)?,
            ),
            "podcast_episode" => SearchResultItem::PodcastEpisode(
                serde_json::from_value(helper.result).map_err(Error::custom)?,
            ),
            "single" => SearchResultItem::Single(
                serde_json::from_value(helper.result).map_err(Error::custom)?,
            ),
            _ => return Err(Error::custom(format!("Unknown type: {}", helper.item_type))),
        };

        Ok(BestResult {
            item_type: helper.item_type,
            text: helper.text,
            result: result_item,
        })
    }
}
