use std::borrow::Cow;

use reqwest::Method;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::account::promo_code::PromoCode,
    YandexMusicClient,
};

pub struct ConsumePromoCodeOptions {
    /// The promo code to be redeemed.
    pub code: String,
    /// The language to use for the request.
    pub language: String,
}

impl ConsumePromoCodeOptions {
    pub fn new(code: impl Into<String>, language: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            language: language.into(),
        }
    }
}

impl Endpoint for ConsumePromoCodeOptions {
    type Options = [(&'static str, String); 2];
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        "account/consume-promo-code".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default().with_form_data([
            ("code", self.code.clone()),
            ("language", self.language.clone()),
        ])
    }
}

impl YandexMusicClient {
    /// Redeems a promo code for the user's account.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the promo code and language.
    ///
    /// ### Returns
    /// * `Result<PromoCode, ClientError>` - The redeemed promo code details or an error if the request fails.
    pub async fn consume_promo_code(
        &self,
        options: &ConsumePromoCodeOptions,
    ) -> Result<PromoCode, crate::ClientError> {
        self.request(options).await
    }
}
