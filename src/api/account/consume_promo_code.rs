use crate::{
    api::RequestPath,
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
    /// Redeems a promo code for the user's account.
    ///
    /// ### Arguments
    /// * `code` - The promo code to be redeemed.
    /// * `language` - The language to use for the request.
    ///
    /// ### Returns
    /// * [PromoCode] - The promo code redeem status.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn consume_promo_code(
        &self,
        code: &str,
        language: &str,
    ) -> Result<PromoCode, crate::ClientError> {
        let response = self
            .post_with_form_str(
                &ConsumePromoCodeRequest {}.path(),
                vec![("code", code), ("language", language)],
            )
            .await?;

        Ok(serde_json::from_value::<PromoCode>(response)?)
    }
}
