use std::collections::HashMap;

use crate::{api::RequestPath, YandexMusicClient};

pub struct AccountExperimentsRequest {}

impl RequestPath for AccountExperimentsRequest {
    fn path(&self) -> String {
        String::from("account/experiments")
    }
}

impl YandexMusicClient {
    /// Returns a list of user's account experiments.
    ///
    /// ### Returns
    /// * [HashMap<String, String>] - A list of user's account experiments.
    /// * [ClientError](crate::ClientError) - If the request fails.
    pub async fn get_account_experiments(
        &self,
    ) -> Result<HashMap<String, String>, crate::ClientError> {
        let response = self.get(&AccountExperimentsRequest {}.path()).await?;

        Ok(serde_json::from_value::<HashMap<String, String>>(response)?)
    }
}
