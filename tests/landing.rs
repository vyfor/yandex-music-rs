#[cfg(test)]
mod landing {
    use yandex_music::{
        api::landing::{get_landing::GetLandingOptions, get_landing_block::GetLandingBlockOptions},
        model::landing::{landing_item::LandingBlockType, LandingType},
        YandexMusicClient,
    };

    #[tokio::test]
    async fn get_landing_block_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        let options = GetLandingBlockOptions::new(LandingBlockType::Podcasts);
        let result = client.get_landing_block(&options).await.unwrap();
        println!("{result:#?}");
    }

    #[tokio::test]
    async fn get_landing_test() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");

        let client = YandexMusicClient::builder(&api_key).build().unwrap();

        let options = GetLandingOptions::new([LandingType::Albums]);
        let result = client.get_landing(&options).await.unwrap();
        println!("{result:#?}");
    }
}
