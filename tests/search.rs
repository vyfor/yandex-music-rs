#[cfg(test)]
mod search {
    use yandex_music::YandexMusicClient;

    #[tokio::test]
    async fn search_suggestion_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client
            .search_suggestion("Molchat Doma".to_string())
            .await
            .unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn search_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client.search("Molchat Doma".to_string()).await.unwrap();
        println!("{result:#?}");
    }
}
