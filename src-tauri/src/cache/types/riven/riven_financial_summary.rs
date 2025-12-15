use entity::{dto::FinancialReport, transaction::TransactionPaginationQueryDto};
use serde::Serialize;
use service::TransactionQuery;
use utils::*;

use crate::DATABASE;
#[derive(Debug, Serialize)]
pub struct RivenFinancialSummary {
    #[serde(flatten)]
    report: FinancialReport,
    last_transactions: Vec<entity::transaction::Model>,
}

impl RivenFinancialSummary {
    pub async fn new(unique_name: impl Into<String>) -> Result<Self, Error> {
        let conn = DATABASE.get().unwrap();

        let transaction_paginate = TransactionQuery::get_all(
            conn,
            TransactionPaginationQueryDto::new(1, -1).set_unique_name(unique_name),
        )
        .await
        .map_err(|e| e.with_location(get_location!()))?;

        Ok(Self {
            report: FinancialReport::from(&transaction_paginate.results),
            last_transactions: transaction_paginate.take_top(5),
        })
    }
}

impl Default for RivenFinancialSummary {
    fn default() -> Self {
        Self {
            report: FinancialReport::default(),
            last_transactions: vec![],
        }
    }
}
