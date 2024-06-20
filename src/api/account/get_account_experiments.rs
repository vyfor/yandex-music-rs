use std::collections::HashMap;

use crate::{
    api::{RequestPath, Response},
    YandexMusicClient,
};

pub struct AccountExperimentsRequest {}

impl RequestPath for AccountExperimentsRequest {
    fn path(&self) -> String {
        String::from("account/experiments")
    }
}

impl YandexMusicClient {
    pub async fn get_account_experiments(
        &self,
    ) -> Result<HashMap<String, String>, crate::ClientError> {
        let response: Response =
            self.get(&AccountExperimentsRequest {}.path()).await?;

        Ok(serde_json::from_value::<HashMap<String, String>>(
            response.result,
        )?)
    }
}
