#[cfg(test)]
mod artist {
    use yandex_music::{
        model::artist_model::artist::SortBy, YandexMusicClient,
    };

    #[tokio::test]
    async fn get_artist_albums_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let artist_id = std::env::var("YANDEX_MUSIC_ARTIST_ID")
            .expect("YANDEX_MUSIC_ARTIST_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client
            .get_artist_albums(artist_id, SortBy::Rating)
            .await
            .unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_artist_track_ids_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let artist_id = std::env::var("YANDEX_MUSIC_ARTIST_ID")
            .expect("YANDEX_MUSIC_ARTIST_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_artist_track_ids(artist_id).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_artist_tracks_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let artist_id = std::env::var("YANDEX_MUSIC_ARTIST_ID")
            .expect("YANDEX_MUSIC_ARTIST_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_artist_tracks(artist_id).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_artist_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let artist_id = std::env::var("YANDEX_MUSIC_ARTIST_ID")
            .expect("YANDEX_MUSIC_ARTIST_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_artist(artist_id).await.unwrap();
        println!("{result:#?}");
    }
}
