#[cfg(test)]
mod album {
    use yandex_music::{
        api::album::{get_album::GetAlbumOptions, get_albums::GetAlbumsOptions},
        YandexMusicClient,
    };

    #[tokio::test]
    async fn get_album_with_tracks_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let album_id = std::env::var("YANDEX_MUSIC_ALBUM_ID")
            .expect("YANDEX_MUSIC_ALBUM_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        let options = GetAlbumOptions::new(album_id).with_tracks();
        let result = client.get_album(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_album_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let album_id = std::env::var("YANDEX_MUSIC_ALBUM_ID")
            .expect("YANDEX_MUSIC_ALBUM_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        let options = GetAlbumOptions::new(album_id);
        let result = client.get_album(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_albums_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");
        let album_id = std::env::var("YANDEX_MUSIC_ALBUM_ID")
            .expect("YANDEX_MUSIC_ALBUM_ID must be set")
            .parse()
            .unwrap();

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        let options = GetAlbumsOptions::new([album_id]);
        let result = client.get_albums(&options).await.unwrap();
        println!("{result:#?}");
    }
}
