#[cfg(test)]
mod track {
    use yandex_music::{
        model::info_model::lyrics::LyricsFormat, YandexMusicClient,
    };

    #[tokio::test]
    async fn get_disliked_tracks_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let user_id = std::env::var("YANDEX_MUSIC_USER_ID")
            .expect("YANDEX_MUSIC_USER_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_disliked_tracks(user_id).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_track_download_info_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let track_id = std::env::var("YANDEX_MUSIC_TRACK_ID")
            .expect("YANDEX_MUSIC_TRACK_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_track_download_info(track_id).await.unwrap();
        println!("{result:#?}");

        let result = result
            .first()
            .unwrap()
            .get_direct_link(&client.client)
            .await
            .unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_liked_tracks_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let user_id = std::env::var("YANDEX_MUSIC_USER_ID")
            .expect("YANDEX_MUSIC_USER_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_liked_tracks(user_id).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_lyrics_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let track_id = std::env::var("YANDEX_MUSIC_TRACK_ID")
            .expect("YANDEX_MUSIC_TRACK_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client
            .get_lyrics(track_id, LyricsFormat::TEXT)
            .await
            .unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_similar_tracks_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let track_id = std::env::var("YANDEX_MUSIC_TRACK_ID")
            .expect("YANDEX_MUSIC_TRACK_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_similar_tracks(track_id).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_track_supplement_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let track_id = std::env::var("YANDEX_MUSIC_TRACK_ID")
            .expect("YANDEX_MUSIC_TRACK_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_track_supplement(track_id).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_track_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let track_id = std::env::var("YANDEX_MUSIC_TRACK_ID")
            .expect("YANDEX_MUSIC_TRACK_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_track(track_id).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_tracks_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let track_id = std::env::var("YANDEX_MUSIC_TRACK_ID")
            .expect("YANDEX_MUSIC_TRACK_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_tracks(&[track_id], true).await.unwrap();
        println!("{result:#?}");
    }
}
