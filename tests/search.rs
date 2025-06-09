#[cfg(test)]
mod search {
    use yandex_music::YandexMusicClient;

    #[tokio::test]
    async fn search_suggestion_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::search::get_search_suggestion::GetSearchSuggestionOptions;
        let options = GetSearchSuggestionOptions::new("Ramm");
        let result = client.get_search_suggestion(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn search_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        use yandex_music::api::search::get_search::SearchOptions;
        let options = SearchOptions::new("Rammstein");
        let result = client.search(&options).await.unwrap();
        println!("{result:#?}");
    }
}
