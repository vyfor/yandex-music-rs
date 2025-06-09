use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub title: String,
    pub cover: Option<String>,
    pub embed_url: Option<String>,
    pub provider: Option<String>,
    pub provider_video_id: Option<String>,
    pub youtube_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration: Option<i32>,
    pub text: Option<String>,
    pub html_auto_play_video_player: Option<String>,
    #[serde(default)]
    pub regions: Vec<i32>,
}
