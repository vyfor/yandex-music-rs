#[cfg(test)]
mod playlist {
    use yandex_music::YandexMusicClient;

    #[tokio::test]
    async fn get_all_playlists_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let user_id = std::env::var("YANDEX_MUSIC_USER_ID")
            .expect("YANDEX_MUSIC_USER_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::playlist::get_all_playlists::GetAllPlaylistsOptions;
        let options = GetAllPlaylistsOptions::new(user_id);
        let result = client.get_all_playlists(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_playlist_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let user_id = std::env::var("YANDEX_MUSIC_USER_ID")
            .expect("YANDEX_MUSIC_USER_ID must be set")
            .parse()
            .unwrap();
        let playlist_kind = std::env::var("YANDEX_MUSIC_PLAYLIST_KIND")
            .expect("YANDEX_MUSIC_PLAYLIST_KIND must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::playlist::get_playlist::GetPlaylistOptions;
        let options = GetPlaylistOptions::new(user_id, playlist_kind);
        let result = client.get_playlist(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_playlists_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let user_id = std::env::var("YANDEX_MUSIC_USER_ID")
            .expect("YANDEX_MUSIC_USER_ID must be set")
            .parse()
            .unwrap();
        let playlist_kind = std::env::var("YANDEX_MUSIC_PLAYLIST_KIND")
            .expect("YANDEX_MUSIC_PLAYLIST_KIND must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::playlist::get_playlists::GetPlaylistsOptions;
        let options = GetPlaylistsOptions::new(user_id)
            .kinds(vec![playlist_kind])
            .with_tracks(true);
        let result = client.get_playlists(&options).await.unwrap();
        println!("{result:#?}");

        let options = GetPlaylistsOptions::new(user_id)
            .kinds(vec![playlist_kind])
            .with_tracks(false);
        let result = client.get_playlists(&options).await.unwrap();

        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_recommendations_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let user_id = std::env::var("YANDEX_MUSIC_USER_ID")
            .expect("YANDEX_MUSIC_USER_ID must be set")
            .parse()
            .unwrap();
        let playlist_kind = std::env::var("YANDEX_MUSIC_PLAYLIST_KIND")
            .expect("YANDEX_MUSIC_PLAYLIST_KIND must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::playlist::get_recommendations::GetRecommendationsOptions;
        let options = GetRecommendationsOptions::new(user_id, playlist_kind);
        let result = client.get_recommendations(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_tagged_playlist_ids_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::playlist::get_tagged_playlists::GetTaggedPlaylistsOptions;
        let options = GetTaggedPlaylistsOptions::new("rock".to_string());
        let result = client.get_tagged_playlist_ids(&options).await.unwrap();
        println!("{result:#?}");
    }
}
