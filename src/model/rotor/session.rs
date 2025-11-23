use serde::Deserialize;

use crate::model::track::{Track, TrackParameters};

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    #[serde(default)]
    pub accepted_seeds: Vec<SessionSeed>,
    pub batch_id: String,
    pub description_seed: Option<SessionSeed>,
    #[serde(default)]
    pub offline_recommender_data: Vec<u32>,
    pub pumpkin: bool,
    pub radio_session_id: Option<String>,
    pub sequence: Vec<SequenceItem>,
    pub terminated: bool,
    pub wave: Option<SessionWave>,
    pub unknown_session: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionSeed {
    pub tag: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceItem {
    pub liked: bool,
    pub track: Track,
    pub track_parameters: TrackParameters,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionWave {
    pub description: String,
    pub id_for_from: String,
    pub name: String,
    pub seeds: Vec<String>,
    pub station_id: String,
}
