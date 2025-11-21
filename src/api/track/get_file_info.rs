use std::borrow::Cow;

use reqwest::Method;
use serde::Deserialize;

pub use crate::model::info::file_info::{Codec, TrackFileInfo};
use crate::{
    api::{utils::create_file_info_sign, Endpoint},
    client::request::RequestOptions,
    error::ClientError,
    YandexMusicClient,
};

pub struct GetFileInfoOptions {
    pub track_id: String,
    pub quality: String,
    pub codec: Codec,
    pub is_encrypted: bool,
}

impl GetFileInfoOptions {
    pub fn new(track_id: impl Into<String>) -> Self {
        Self {
            track_id: track_id.into(),
            quality: "lossless".to_string(),
            codec: Codec::Mp3,
            is_encrypted: false,
        }
    }

    pub fn quality(mut self, quality: impl Into<String>) -> Self {
        self.quality = quality.into();
        self
    }

    pub fn codec(mut self, codec: Codec) -> Self {
        self.codec = codec;
        self
    }

    pub fn is_encrypted(mut self, is_encrypted: bool) -> Self {
        self.is_encrypted = is_encrypted;
        self
    }
}

impl Endpoint for GetFileInfoOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        let transport = if self.is_encrypted { "encraw" } else { "raw" };
        let (ts, sign) = create_file_info_sign(
            &self.track_id,
            &self.quality,
            &self.codec.to_string(),
            transport,
        );

        let mut serializer = url::form_urlencoded::Serializer::new(String::from("get-file-info?"));
        serializer.append_pair("ts", &ts);
        serializer.append_pair("trackId", &self.track_id);
        serializer.append_pair("quality", &self.quality);
        serializer.append_pair("codecs", &self.codec.to_string());
        serializer.append_pair("transports", transport);
        serializer.append_pair("sign", &sign);

        serializer.finish().into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetFileInfoResult {
    download_info: TrackFileInfo,
}

impl YandexMusicClient {
    /// Get track file info (direct links) with support for high quality/lossless.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the track ID and codec preferences.
    ///
    /// ### Returns
    /// * [`TrackFileInfo`] - The track file info containing direct download URLs.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_file_info(
        &self,
        options: &GetFileInfoOptions,
    ) -> Result<TrackFileInfo, ClientError> {
        let result: GetFileInfoResult = self.request(options).await?;
        Ok(result.download_info)
    }
}
