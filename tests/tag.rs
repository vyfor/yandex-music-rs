#[cfg(test)]
mod tag {
    use yandex_music::YandexMusicClient;

    #[tokio::test]
    async fn get_tagged_playlist_ids_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client
            .get_tagged_playlist_ids("rock".to_string())
            .await
            .unwrap();
        println!("{result:#?}");
    }
}
