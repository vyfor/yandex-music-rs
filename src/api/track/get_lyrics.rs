use crate::{
    api::{utils::create_sign, RequestPath},
    model::info_model::lyrics::{LyricsFormat, TrackLyrics},
    YandexMusicClient,
};

pub struct LyricsRequest {
    pub track_id: String,
    pub format: LyricsFormat,
    pub timestamp: u64,
    pub sign: String,
}

impl LyricsRequest {
    pub fn new(track_id: String, format: LyricsFormat) -> Self {
        let (timestamp, sign) = create_sign(track_id.clone());

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
    /// Get track lyrics.
    ///
    /// ### Arguments
    /// * `track_id` - The ID of the track.
    /// * `format` - The format of the lyrics.
    ///
    /// ### Returns
    /// * [TrackLyrics] - The track lyrics.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_lyrics(
        &self,
        track_id: String,
        format: LyricsFormat,
    ) -> Result<TrackLyrics, crate::ClientError> {
        let response = self
            .get(&LyricsRequest::new(track_id, format).path())
            .await?;

        Ok(serde_json::from_value::<TrackLyrics>(response)?)
    }
}
