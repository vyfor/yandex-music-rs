#[cfg(test)]
mod account {
    use yandex_music::YandexMusicClient;

    #[tokio::test]
    async fn get_account_experiments_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_account_experiments().await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_account_settings_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_account_settings().await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_account_status_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_account_status().await.unwrap();
        println!("{result:#?}");
    }
}
