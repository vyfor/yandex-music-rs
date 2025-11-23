use std::borrow::Cow;

use reqwest::Method;
use serde::Deserialize;

pub use crate::model::info::file_info::{Codec, TrackFileInfo};
use crate::{
    api::{
        utils::{create_file_info_batch_sign, JoinDisplay},
        Endpoint,
    },
    client::request::RequestOptions,
    error::ClientError,
    model::info::file_info::Quality,
    YandexMusicClient,
};

pub struct GetFileInfoBatchOptions {
    pub track_ids: Vec<String>,
    pub quality: Quality,
    pub codecs: Vec<Codec>,
    pub is_encrypted: bool,
}

impl GetFileInfoBatchOptions {
    pub fn new<I, S>(track_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            track_ids: track_ids.into_iter().map(Into::into).collect(),
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

impl Endpoint for GetFileInfoBatchOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        let transport = if self.is_encrypted { "encraw" } else { "raw" };

        let quality = self.quality.to_string();
        let (ts, sign) = create_file_info_batch_sign(
            &self.track_ids,
            &quality,
            &self.codecs.concatenated(),
            transport,
        );

        let mut serializer =
            url::form_urlencoded::Serializer::new(String::from("get-file-info/batch?"));
        serializer.append_pair("ts", &ts);
        serializer.append_pair("trackIds", &self.track_ids.delimited(","));
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
struct GetFileInfoBatchResult {
    download_infos: Vec<TrackFileInfo>,
}

impl YandexMusicClient {
    /// Get multiple track file infos (direct links) with support for high quality/lossless.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the track ID and codec preferences.
    ///
    /// ### Returns
    /// * [`Vec<TrackFileInfo>`] - The vector of track file infos containing direct download URLs.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_file_info_batch(
        &self,
        options: &GetFileInfoBatchOptions,
    ) -> Result<Vec<TrackFileInfo>, ClientError> {
        let result: GetFileInfoBatchResult = self.request(options).await?;
        Ok(result.download_infos)
    }
}
