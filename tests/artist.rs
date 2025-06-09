#[cfg(test)]
mod artist {
    use yandex_music::YandexMusicClient;

    #[tokio::test]
    async fn get_artist_albums_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let artist_id =
            std::env::var("YANDEX_MUSIC_ARTIST_ID").expect("YANDEX_MUSIC_ARTIST_ID must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::artist::get_artist_albums::GetArtistAlbumsOptions;
        let options = GetArtistAlbumsOptions::new(artist_id);
        let result = client.get_artist_albums(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_artist_track_ids_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let artist_id =
            std::env::var("YANDEX_MUSIC_ARTIST_ID").expect("YANDEX_MUSIC_ARTIST_ID must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::artist::get_artist_track_ids::ArtistTrackIdsOptions;
        let options = ArtistTrackIdsOptions::new(artist_id);
        let result = client.get_artist_track_ids(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_artist_tracks_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let artist_id =
            std::env::var("YANDEX_MUSIC_ARTIST_ID").expect("YANDEX_MUSIC_ARTIST_ID must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::artist::get_artist_tracks::ArtistTracksOptions;
        let options = ArtistTracksOptions::new(artist_id);
        let result = client.get_artist_tracks(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_artist_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let artist_id =
            std::env::var("YANDEX_MUSIC_ARTIST_ID").expect("YANDEX_MUSIC_ARTIST_ID must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::artist::get_artist::GetArtistOptions;
        let options = GetArtistOptions::new(artist_id);
        let result = client.get_artist(&options).await.unwrap();
        println!("{result:#?}");
    }
}
