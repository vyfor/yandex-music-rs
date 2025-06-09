use std::borrow::Cow;

use reqwest::Method;
use serde_json::Value;

use crate::{
    api::Endpoint,
    client::request::RequestOptions,
    error::{ClientError, YandexMusicError},
    YandexMusicClient,
};

/// Request for checking if the user has completed the wizard.
pub struct GetWizardIsPassedOptions;

impl Default for GetWizardIsPassedOptions {
    fn default() -> Self {
        Self
    }
}

impl Endpoint for GetWizardIsPassedOptions {
    type Options = ();
    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "feed/wizard/is-passed".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default()
    }
}

impl YandexMusicClient {
    /// Check if the user has completed the wizard.
    ///
    /// ### Returns
    /// * `Result<bool, ClientError>` - `true` if the wizard has been completed, `false` otherwise.
    pub async fn get_wizard_is_passed(&self) -> Result<bool, crate::ClientError> {
        let response = self.request::<Value, _>(&GetWizardIsPassedOptions).await?;

        response["isWizardPassed"]
            .as_bool()
            .ok_or(ClientError::YandexMusicError {
                error: YandexMusicError {
                    name: "InvalidValue".to_string(),
                    message: Some("isWizardPassed is not a boolean".to_string()),
                },
            })
    }
}
