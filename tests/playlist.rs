#[cfg(test)]
mod playlist {
    use yandex_music::{
        api::playlist::get_playlists::PlaylistsRequest, YandexMusicClient,
    };

    #[tokio::test]
    async fn get_all_playlists_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let user_id = std::env::var("YANDEX_MUSIC_USER_ID")
            .expect("YANDEX_MUSIC_USER_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_all_playlists(user_id).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_playlist_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let user_id = std::env::var("YANDEX_MUSIC_USER_ID")
            .expect("YANDEX_MUSIC_USER_ID must be set")
            .parse()
            .unwrap();
        let playlist_kind = std::env::var("YANDEX_MUSIC_PLAYLIST_KIND")
            .expect("YANDEX_MUSIC_PLAYLIST_KIND must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client.get_playlist(user_id, playlist_kind).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_playlists_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let user_id = std::env::var("YANDEX_MUSIC_USER_ID")
            .expect("YANDEX_MUSIC_USER_ID must be set")
            .parse()
            .unwrap();
        let playlist_kind = std::env::var("YANDEX_MUSIC_PLAYLIST_KIND")
            .expect("YANDEX_MUSIC_PLAYLIST_KIND must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client
            .get_playlists(
                &PlaylistsRequest::new(user_id)
                    .kinds(vec![playlist_kind])
                    .with_tracks(true),
            )
            .await
            .unwrap();

        println!("{result:#?}");

        let result = client
            .get_playlists(
                &PlaylistsRequest::new(user_id)
                    .kinds(vec![playlist_kind])
                    .with_tracks(false),
            )
            .await
            .unwrap();

        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_recommendations_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");
        let user_id = std::env::var("YANDEX_MUSIC_USER_ID")
            .expect("YANDEX_MUSIC_USER_ID must be set")
            .parse()
            .unwrap();
        let playlist_kind = std::env::var("YANDEX_MUSIC_PLAYLIST_KIND")
            .expect("YANDEX_MUSIC_PLAYLIST_KIND must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::new(&api_key);

        let result = client
            .get_recommendations(user_id, playlist_kind)
            .await
            .unwrap();
        println!("{result:#?}");
    }
}
