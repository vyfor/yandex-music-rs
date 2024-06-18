#[cfg(test)]
mod info {
    use yandex_music::YandexMusicClient;

    #[tokio::test]
    async fn get_track_lyrics_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let track_id = std::env::var("YANDEX_MUSIC_TRACK_ID")
            .expect("YANDEX_MUSIC_TRACK_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_lyrics(track_id).await.unwrap();
        println!("{result:#?}");
    }
}
