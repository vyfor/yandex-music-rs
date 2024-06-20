use crate::{
    api::{RequestPath, Response},
    YandexMusicClient,
};

pub struct IsWizardPassedRequest {}

impl RequestPath for IsWizardPassedRequest {
    fn path(&self) -> String {
        String::from("feed/wizard/is-passed")
    }
}

impl YandexMusicClient {
    pub async fn get_is_wizard_passed(
        &self,
    ) -> Result<bool, crate::ClientError> {
        let mut response: Response =
            self.get(&IsWizardPassedRequest {}.path()).await?;

        Ok(serde_json::from_value::<bool>(
            response.result["isWizardPassed"].take(),
        )?)
    }
}
