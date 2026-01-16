#[cfg(test)]
mod collection {
    use yandex_music::{api::collection::sync::CollectionSyncOptions, YandexMusicClient};

    #[tokio::test]
    async fn collection_sync_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        let options = CollectionSyncOptions::all();
        let result = client.collection_sync(&options).await.unwrap();
        println!("{result:#?}");
    }
}
