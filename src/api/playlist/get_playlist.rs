use crate::{
    api::{RequestPath, Response},
    error::ClientError,
    model::playlist::Playlist,
    YandexMusicClient,
};

pub struct PlaylistRequest {
    pub user_id: i32,
    pub kind: i32,
}

impl PlaylistRequest {
    pub fn new(user_id: i32, kind: i32) -> Self {
        Self { user_id, kind }
    }
}

impl RequestPath for PlaylistRequest {
    fn path(&self) -> String {
        format!("/users/{}/playlists/{}", self.user_id, self.kind)
    }
}

impl YandexMusicClient {
    pub async fn get_playlist(
        &self,
        user_id: i32,
        kind: i32,
    ) -> Result<Playlist, ClientError> {
        let response: Response = self
            .get(&PlaylistRequest::new(user_id, kind).path())
            .await?;

        Ok(serde_json::from_value::<Playlist>(response.result)?)
    }
}

#[cfg(test)]
#[tokio::test]
async fn get_playlist_test() {
    dotenv::dotenv().ok();
    let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
        .expect("YANDEX_MUSIC_TOKEN must be set");
    let user_id = std::env::var("YANDEX_MUSIC_USER_ID")
        .expect("YANDEX_MUSIC_USER_ID must be set")
        .parse()
        .unwrap();
    let playlist_kind = std::env::var("YANDEX_MUSIC_PLAYLIST_KIND")
        .expect("YANDEX_MUSIC_PLAYLIST_KIND must be set")
        .parse()
        .unwrap();

    let client = crate::YandexMusicClient::new(&api_key);

    let result = client.get_playlist(user_id, playlist_kind).await.unwrap();
    println!("{result:#?}");
}
