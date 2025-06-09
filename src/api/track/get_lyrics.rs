use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::{utils::create_sign, Endpoint},
    client::request::RequestOptions,
    model::info::lyrics::{LyricsFormat, TrackLyrics},
    YandexMusicClient,
};

/// Request for retrieving lyrics for a track.
pub struct GetLyricsOptions {
    /// The ID of the track to get lyrics for.
    pub track_id: String,
    /// The desired format for the lyrics.
    pub format: LyricsFormat,
    /// Timestamp for the request signature.
    pub timestamp: u64,
    /// Cryptographic signature for the request.
    pub sign: String,
}

impl GetLyricsOptions {
    /// Create a new request to get lyrics for a track.
    pub fn new(track_id: impl Into<String>, format: LyricsFormat) -> Self {
        let track_id = track_id.into();
        let (timestamp, sign) = create_sign(track_id.as_str());

        Self {
            track_id,
            format,
            timestamp,
            sign,
        }
    }
}

impl Endpoint for GetLyricsOptions {
    type Options = [(&'static str, String); 3];
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("tracks/{}/lyrics", self.track_id,).into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Retrieve lyrics for a specific track.
    ///
    /// ### Arguments
    /// * `options` - The request options containing track ID and lyrics format.
    ///
    /// ### Returns
    /// * `Result<TrackLyrics, ClientError>` - The track lyrics or an error if the request fails.
    pub async fn get_lyrics(
        &self,
        options: &GetLyricsOptions,
    ) -> Result<TrackLyrics, crate::ClientError> {
        let (timestamp, sign) = create_sign(options.track_id.as_str());
        let url = format!(
            "tracks/{}/lyrics?format={}&timeStamp={}&sign={}",
            options.track_id,
            options.format,
            timestamp,
            sign.replace("+", "%2B")
        );

        self.request_with_url::<TrackLyrics, _>(url, options).await
    }
}
