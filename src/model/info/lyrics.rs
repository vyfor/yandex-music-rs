use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackLyrics {
    pub download_url: String,
    pub lyric_id: i32,
    pub external_lyric_id: String,
    pub writers: Vec<String>,
    pub major: LyricsMajor,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsMajor {
    pub id: i32,
    pub name: String,
}

/// The format of the lyrics.
/// - `TEXT`: Plain text lyrics.
/// - `LRC`: Lyrics with timestamps attached.
pub enum LyricsFormat {
    TEXT,
    LRC,
}

impl Display for LyricsFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LyricsFormat::TEXT => write!(f, "TEXT"),
            LyricsFormat::LRC => write!(f, "LRC"),
        }
    }
}
