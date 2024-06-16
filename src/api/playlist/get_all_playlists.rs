use crate::api::RequestPath;

pub struct AllPlaylistsRequest {
    pub user_id: i32,
}

impl AllPlaylistsRequest {
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
    }
}

impl RequestPath for AllPlaylistsRequest {
    fn path(&self) -> String {
        format!("/users/{}/playlists/list", self.user_id)
    }
}

#[tokio::test]
async fn get_all_playlists_test() {
    dotenv::dotenv().ok();
    let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
        .expect("YANDEX_MUSIC_TOKEN must be set");
    let user_id = std::env::var("YANDEX_MUSIC_USER_ID")
        .expect("YANDEX_MUSIC_USER_ID must be set")
        .parse()
        .unwrap();

    let client = crate::YandexMusicClient::new(&api_key);

    let result = client.get_all_playlists(user_id).await.unwrap();
    println!("{result:#?}");
}
