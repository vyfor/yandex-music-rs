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
    Flac,
    Aac,
    HeAac,
    Mp3,
    FlacMp4,
    AacMp4,
    HeAacMp4,
}

impl Codec {
    pub fn all() -> [Self; 7] {
        [
            Codec::Flac,
            Codec::Aac,
            Codec::HeAac,
            Codec::Mp3,
            Codec::FlacMp4,
            Codec::AacMp4,
            Codec::HeAacMp4,
        ]
    }
}

impl std::fmt::Display for Codec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Codec::Flac => "flac",
            Codec::FlacMp4 => "flac-mp4",
            Codec::Aac => "aac",
            Codec::AacMp4 => "aac-mp4",
            Codec::HeAac => "he-aac",
            Codec::HeAacMp4 => "he-aac-mp4",
            Codec::Mp3 => "mp3",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quality {
    Lossless,
    Normal,
    Low,
}

impl std::fmt::Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Quality::Lossless => "lossless",
            Quality::Normal => "nq",
            Quality::Low => "lq",
        };
        write!(f, "{}", s)
    }
}
