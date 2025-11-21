use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackFileInfo {
    pub bitrate: u32,
    pub codec: String,
    pub gain: bool,
    pub quality: String,
    pub real_id: String,
    pub size: u64,
    pub track_id: String,
    pub transport: String,
    pub url: String,
    pub urls: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Codec {
    Mp3,
    FlacMp4,
    AacMp4,
    HeAacMp4,
}

impl std::fmt::Display for Codec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Codec::Mp3 => "mp3",
            Codec::FlacMp4 => "flac-mp4",
            Codec::AacMp4 => "aac-mp4",
            Codec::HeAacMp4 => "he-aac-mp4",
        };
        write!(f, "{}", s)
    }
}
