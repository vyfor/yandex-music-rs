#[cfg(test)]
mod playlist {
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
}
