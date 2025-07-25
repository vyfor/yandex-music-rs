use reqwest::Client;
use serde::Deserialize;

use crate::error::ClientError;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackDownloadInfo {
    pub codec: String,
    pub gain: bool,
    pub preview: bool,
    pub download_info_url: String,
    pub direct: bool,
    pub bitrate_in_kbps: u32,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
struct DownloadInfo {
    pub host: String,
    pub path: String,
    pub ts: String,
    pub region: String,
    pub s: String,
}

impl DownloadInfo {
    const SIGN_SALT: &'static str = "XGRlBW9FXlekgbPrRHuSiA";

    pub fn get_direct_link(&self) -> String {
        let sign = md5::compute(
            (format!(
                "{}{}{}",
                Self::SIGN_SALT,
                self.path.get(1..).unwrap(),
                self.s
            ))
            .as_bytes(),
        );

        format!(
            "https://{}/get-mp3/{:x}/{}{}",
            self.host, sign, self.ts, self.path
        )
    }
}

impl TrackDownloadInfo {
    pub async fn get_direct_link(&self, client: &Client) -> Result<String, ClientError> {
        let xml = client
            .get(&self.download_info_url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let download_info: DownloadInfo = serde_xml_rs::from_str(&xml)?;

        Ok(download_info.get_direct_link())
    }
}
