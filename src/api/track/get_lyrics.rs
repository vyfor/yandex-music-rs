use crate::{
    api::{utils::create_sign, RequestPath, Response},
    model::info_model::lyrics::{LyricsFormat, TrackLyrics},
    YandexMusicClient,
};

pub struct LyricsRequest {
    pub track_id: i32,
    pub format: LyricsFormat,
    pub timestamp: u64,
    pub sign: String,
}

impl LyricsRequest {
    pub fn new(track_id: i32, format: LyricsFormat) -> Self {
        let (timestamp, sign) = create_sign(track_id);

        Self {
            track_id,
            format,
            timestamp,
            sign,
        }
    }
}

impl RequestPath for LyricsRequest {
    fn path(&self) -> String {
        let base_path = format!(
            "tracks/{}/lyrics?format={}&timeStamp={}&sign={}",
            self.track_id,
            self.format,
            self.timestamp,
            self.sign.replace('+', "%2B")
        );

        base_path
    }
}

impl YandexMusicClient {
    pub async fn get_lyrics(
        &self,
        track_id: i32,
        format: LyricsFormat,
    ) -> Result<TrackLyrics, crate::ClientError> {
        let response: Response = self
            .get(&LyricsRequest::new(track_id, format).path())
            .await?;

        Ok(serde_json::from_value::<TrackLyrics>(response.result)?)
    }
}
