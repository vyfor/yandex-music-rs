use crate::{
    api::{RequestPath, Response},
    model::account_model::promo_code::PromoCode,
    YandexMusicClient,
};

pub struct ConsumePromoCodeRequest {}

impl RequestPath for ConsumePromoCodeRequest {
    fn path(&self) -> String {
        String::from("account/consume-promo-code")
    }
}

impl YandexMusicClient {
    pub async fn consume_promo_code(
        &self,
        code: &str,
        language: &str,
    ) -> Result<PromoCode, crate::ClientError> {
        let response: Response = self
            .post_with_form_str(
                &ConsumePromoCodeRequest {}.path(),
                vec![("code", code), ("language", language)],
            )
            .await?;

        Ok(serde_json::from_value::<PromoCode>(response.result)?)
    }
}
