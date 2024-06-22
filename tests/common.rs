#[cfg(test)]
mod common {
    use yandex_music::YandexMusicClient;

    #[tokio::test]
    async fn get_feed_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_feed().await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_genres_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_genres().await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_non_music_catalogue_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_non_music_catalogue().await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_permission_alerts_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_permission_alerts().await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_settings_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_settings().await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_is_wizard_passed_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_is_wizard_passed().await.unwrap();
        println!("{result:#?}");
    }
}
