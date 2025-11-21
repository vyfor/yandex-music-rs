#[cfg(test)]
mod track {
    use yandex_music::{model::info::lyrics::LyricsFormat, YandexMusicClient};

    #[tokio::test]
    async fn get_disliked_tracks_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let user_id = std::env::var("YANDEX_MUSIC_USER_ID")
            .expect("YANDEX_MUSIC_USER_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::track::get_disliked_tracks::GetDislikedTracksOptions;
        let options = GetDislikedTracksOptions::new(user_id);
        let result = client.get_disliked_tracks(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_track_download_info_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let track_id =
            std::env::var("YANDEX_MUSIC_TRACK_ID").expect("YANDEX_MUSIC_TRACK_ID must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::track::get_download_info::GetDownloadInfoOptions;
        let options = GetDownloadInfoOptions::new(track_id);
        let result = client.get_download_info(&options).await.unwrap();
        println!("{result:#?}");

        let result = result
            .first()
            .unwrap()
            .get_direct_link(&client.inner)
            .await
            .unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_liked_tracks_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let user_id = std::env::var("YANDEX_MUSIC_USER_ID")
            .expect("YANDEX_MUSIC_USER_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::track::get_liked_tracks::GetLikedTracksOptions;
        let options = GetLikedTracksOptions::new(user_id);
        let result = client.get_liked_tracks(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_lyrics_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let track_id =
            std::env::var("YANDEX_MUSIC_TRACK_ID").expect("YANDEX_MUSIC_TRACK_ID must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::track::get_lyrics::GetLyricsOptions;
        let options = GetLyricsOptions::new(track_id, LyricsFormat::TEXT);
        let result = client.get_lyrics(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_similar_tracks_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let track_id =
            std::env::var("YANDEX_MUSIC_TRACK_ID").expect("YANDEX_MUSIC_TRACK_ID must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::track::get_similar_tracks::GetSimilarTracksOptions;
        let options = GetSimilarTracksOptions::new(track_id);
        let result = client.get_similar_tracks(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_track_supplement_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let track_id =
            std::env::var("YANDEX_MUSIC_TRACK_ID").expect("YANDEX_MUSIC_TRACK_ID must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::track::get_track_supplement::GetTrackSupplementOptions;
        let options = GetTrackSupplementOptions::new(track_id);
        let result = client.get_track_supplement(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_track_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let track_id =
            std::env::var("YANDEX_MUSIC_TRACK_ID").expect("YANDEX_MUSIC_TRACK_ID must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::track::get_tracks::GetTracksOptions;
        let options = GetTracksOptions::new(vec![track_id]);
        let result = client.get_tracks(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_tracks_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let track_id =
            std::env::var("YANDEX_MUSIC_TRACK_ID").expect("YANDEX_MUSIC_TRACK_ID must be set");
        let track_ids = vec![track_id];
        let options = yandex_music::api::track::get_tracks::GetTracksOptions::new(track_ids);

        let client = YandexMusicClient::builder(&api_key).build().unwrap();
        let result = client.get_tracks(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_track_file_info_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let track_id =
            std::env::var("YANDEX_MUSIC_TRACK_ID").expect("YANDEX_MUSIC_TRACK_ID must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::track::get_file_info::GetFileInfoOptions;
        let options = GetFileInfoOptions::new(track_id)
            .codec(yandex_music::model::info::file_info::Codec::FlacMp4);
        let result = client.get_file_info(&options).await.unwrap();
        println!("{result:#?}");
    }
}
