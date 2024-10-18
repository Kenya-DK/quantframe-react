use eyre::eyre;
use serde_json::{json, Value};

use crate::{
    qf_client::client::QFClient,
    utils::{
        enums::log_level::LogLevel,
        modules::error::{ApiResult, AppError},
    },
};
#[derive(Clone, Debug)]
pub struct TransactionModule {
    pub client: QFClient,
    component: String,
}

impl TransactionModule {
    pub fn new(client: QFClient) -> Self {
        TransactionModule {
            client,
            component: "Transaction".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    pub async fn create_transaction(
        &self,
        transaction: &entity::transaction::transaction::Model,
    ) -> Result<(), AppError> {
        let settings = self.client.settings.lock()?.clone();
        let analytics = settings.analytics;

        if !analytics.transaction {
            return Ok(());
        }

        match self
            .client
            .post::<Value>("stats/transaction", json!(transaction))
            .await
        {
            Ok(ApiResult::Success(_, _)) => {
                return Ok(());
            }
            Ok(ApiResult::Error(e, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("Create"),
                    e,
                    eyre!("There was an error creating the transaction"),
                    LogLevel::Error,
                ));
            }
            Err(e) => return Err(e),
        }
    }
    pub async fn delete_transaction(&self, transaction_id: i64) -> Result<(), AppError> {
        let settings = self.client.settings.lock()?.clone();
        let analytics = settings.analytics;

        if !analytics.transaction {
            return Ok(());
        }

        match self
            .client
            .delete::<Value>(&format!("stats/transaction/{}", transaction_id))
            .await
        {
            Ok(ApiResult::Success(_, _)) => {
                return Ok(());
            }
            Ok(ApiResult::Error(e, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("Delete"),
                    e,
                    eyre!("There was an error deleting the transaction"),
                    LogLevel::Error,
                ));
            }
            Err(e) => return Err(e),
        }
    }
    pub async fn update_transaction(
        &self,
        transaction: &entity::transaction::transaction::Model,
    ) -> Result<(), AppError> {
        let settings = self.client.settings.lock()?.clone();
        let analytics = settings.analytics;

        if !analytics.transaction {
            return Ok(());
        }

        match self
            .client
            .put::<Value>(
                &format!("stats/transaction/{}", transaction.id),
                Some(json!(transaction)),
            )
            .await
        {
            Ok(ApiResult::Success(_, _)) => {
                return Ok(());
            }
            Ok(ApiResult::Error(e, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("Update"),
                    e,
                    eyre!("There was an error updating the transaction"),
                    LogLevel::Error,
                ));
            }
            Err(e) => return Err(e),
        }
    }
}
