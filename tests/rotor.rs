#[cfg(test)]
mod rotor {
    use yandex_music::YandexMusicClient;

    #[tokio::test]
    async fn get_all_stations_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::rotor::get_all_stations::GetAllStationsOptions;
        let options = GetAllStationsOptions::default();
        let result = client.get_all_stations(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_station_account_status_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        let result = client.get_station_account_status().await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_station_tracks_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::rotor::get_station_tracks::GetStationTracksOptions;
        let options = GetStationTracksOptions::new("user:onyourwave");
        let result = client.get_station_tracks(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_station_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        let options =
            yandex_music::api::rotor::get_station::GetStationOptions::new("user:onyourwave");
        let result = client.get_station(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_stations_dashboard_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        let result = client.get_stations_dashboard().await.unwrap();
        println!("{result:#?}");
    }
}
