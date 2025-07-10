#[cfg(test)]
mod playlist {
    use yandex_music::{
        api::playlist::{get_playlist::GetPlaylistOptions, modify_playlist::ModifyPlaylistOptions},
        model::{
            playlist::modify::{Diff, DiffOp},
            track::TrackShort,
        },
        YandexMusicClient,
    };

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

    #[tokio::test]
    async fn modify_playlist_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let user_id = std::env::var("YANDEX_MUSIC_USER_ID")
            .expect("YANDEX_MUSIC_USER_ID must be set")
            .parse()
            .unwrap();
        let track_id =
            std::env::var("YANDEX_MUSIC_TRACK_ID").expect("YANDEX_MUSIC_TRACK_ID must be set");
        let album_id =
            std::env::var("YANDEX_MUSIC_ALBUM_ID").expect("YANDEX_MUSIC_ALBUM_ID must be set");
        let kind = std::env::var("YANDEX_MUSIC_PLAYLIST_KIND")
            .expect("YANDEX_MUSIC_PLAYLIST_KIND must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        let playlist = client
            .get_playlist(&GetPlaylistOptions::new(user_id, kind))
            .await
            .unwrap();

        let diff = Diff::new(
            DiffOp::insert(0),
            vec![TrackShort::new(track_id.clone(), Some(album_id.clone()))],
        );
        let options = ModifyPlaylistOptions::new(user_id, kind, &diff, playlist.revision);
        let result = client.modify_playlist(&options).await.unwrap();

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let diff = Diff::new(
            DiffOp::delete(0, 1),
            vec![TrackShort::new(track_id, Some(album_id))],
        );
        let options = ModifyPlaylistOptions::new(user_id, kind, &diff, result.revision);
        client.modify_playlist(&options).await.unwrap();
    }
}
