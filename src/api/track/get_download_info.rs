use crate::{api::RequestPath, YandexMusicClient};

pub struct DownloadInfoRequest {
    pub track_id: i32,
}

impl DownloadInfoRequest {
    pub fn new(track_id: i32) -> Self {
        Self { track_id }
    }
}

impl RequestPath for DownloadInfoRequest {
    fn path(&self) -> String {
        format!("tracks/{}/download-info", self.track_id)
    }
}

#[tokio::test]
async fn get_track_download_info_test() {
    dotenv::dotenv().ok();
    let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
        .expect("YANDEX_MUSIC_TOKEN must be set");
    let track_id = std::env::var("YANDEX_MUSIC_TRACK_ID")
        .expect("YANDEX_MUSIC_TRACK_ID must be set")
        .parse()
        .unwrap();

    let client = YandexMusicClient::new(&api_key);

    let result = client.get_track_download_info(track_id).await.unwrap();
    println!("{result:#?}");

    let result = result
        .first()
        .unwrap()
        .get_direct_link(&client.client)
        .await;
    println!("{result:#?}");
}
