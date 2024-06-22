#[cfg(test)]
mod album {
    use yandex_music::YandexMusicClient;

    #[tokio::test]
    async fn get_album_with_tracks_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let album_id = std::env::var("YANDEX_MUSIC_ALBUM_ID")
            .expect("YANDEX_MUSIC_ALBUM_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_album_with_tracks(album_id).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_album_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let album_id = std::env::var("YANDEX_MUSIC_ALBUM_ID")
            .expect("YANDEX_MUSIC_ALBUM_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_album(album_id).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_albums_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let album_id = std::env::var("YANDEX_MUSIC_ALBUM_ID")
            .expect("YANDEX_MUSIC_ALBUM_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_albums(&[album_id]).await.unwrap();
        println!("{result:#?}");
    }
}
