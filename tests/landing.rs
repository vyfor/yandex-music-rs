#[cfg(test)]
mod landing {
    use yandex_music::{
        model::landing_model::{
            landing::LandingType, landing_item::LandingBlockType,
        },
        YandexMusicClient,
    };

    #[tokio::test]
    async fn get_landing_block_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result = client
            .get_landing_block(LandingBlockType::Podcasts)
            .await
            .unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_landing_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN")
            .expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::new(&api_key);

        let result =
            client.get_landing(vec![LandingType::Albums]).await.unwrap();
        println!("{result:#?}");
    }
}
