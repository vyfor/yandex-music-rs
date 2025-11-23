use std::borrow::Cow;

use reqwest::Method;
use serde::Deserialize;

pub use crate::model::info::file_info::{Codec, TrackFileInfo};
use crate::{
    api::{
        utils::{create_file_info_sign, JoinDisplay},
        Endpoint,
    },
    client::request::RequestOptions,
    error::ClientError,
    model::info::file_info::Quality,
    YandexMusicClient,
};

pub struct GetFileInfoOptions {
    pub track_id: String,
    pub quality: Quality,
    pub codecs: Vec<Codec>,
    pub is_encrypted: bool,
}

impl GetFileInfoOptions {
    pub fn new(track_id: impl Into<String>) -> Self {
        Self {
            track_id: track_id.into(),
            quality: Quality::Lossless,
            codecs: Codec::all().to_vec(),
            is_encrypted: false,
        }
    }

    pub fn quality(mut self, quality: Quality) -> Self {
        self.quality = quality;
        self
    }

    pub fn codecs<I>(mut self, codecs: I) -> Self
    where
        I: IntoIterator<Item = Codec>,
    {
        self.codecs = codecs.into_iter().collect();
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

        let quality = self.quality.to_string();
        let (ts, sign) = create_file_info_sign(
            &self.track_id,
            &quality,
            &self.codecs.concatenated(),
            transport,
        );

        let mut serializer = url::form_urlencoded::Serializer::new(String::from("get-file-info?"));
        serializer.append_pair("ts", &ts);
        serializer.append_pair("trackId", &self.track_id);
        serializer.append_pair("quality", &quality);
        serializer.append_pair("codecs", &self.codecs.delimited(","));
        serializer.append_pair("transports", transport);
        serializer.append_pair("sign", &sign);

        let s = serializer.finish();

        s.into()
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
