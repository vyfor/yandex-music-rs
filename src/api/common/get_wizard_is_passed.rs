use crate::{api::RequestPath, YandexMusicClient};

pub struct IsWizardPassedRequest {}

impl RequestPath for IsWizardPassedRequest {
    fn path(&self) -> String {
        String::from("feed/wizard/is-passed")
    }
}

impl YandexMusicClient {
    /// Get whether the user has passed the wizard.
    ///
    /// ### Returns
    /// * [bool] - Whether the user has passed the wizard.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_is_wizard_passed(
        &self,
    ) -> Result<bool, crate::ClientError> {
        let mut response = self.get(&IsWizardPassedRequest {}.path()).await?;

        Ok(serde_json::from_value::<bool>(
            response["isWizardPassed"].take(),
        )?)
    }
}
