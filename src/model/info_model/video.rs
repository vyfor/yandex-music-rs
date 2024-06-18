use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackVideo {
    pub title: String,
    pub cover: String,
    pub embed_url: String,
    pub provider: String,
    pub provider_video_id: String,
}
