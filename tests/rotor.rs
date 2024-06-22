#[cfg(test)]
mod rotor {
    use yandex_music::YandexMusicClient;

    #[tokio::test]
    async fn get_all_stations_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_all_stations().await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_station_account_status_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_station_account_status().await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_station_tracks_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client
            .get_station_tracks("user:onyourwave".to_string())
            .await
            .unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_station_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client
            .get_station("user:onyourwave".to_string())
            .await
            .unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_stations_dashboard_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_stations_dashboard().await.unwrap();
        println!("{result:#?}");
    }
}
