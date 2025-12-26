use entity::{dto::FinancialReport, stock_riven, transaction::TransactionPaginationQueryDto};
use serde::Serialize;
use service::{StockRivenQuery, TransactionQuery};
use utils::*;

use crate::{utils::modules::states, DATABASE};
#[derive(Debug, Serialize)]
pub struct RivenFinancialSummary {
    #[serde(flatten)]
    report: FinancialReport,
    last_transactions: Vec<entity::transaction::Model>,
    bought_price: i64,
    potential_profit: i64,
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
            bought_price: -1,
            potential_profit: -1,
        })
    }
    pub async fn from_uuid(uuid: impl Into<String>) -> Result<Self, Error> {
        let uuid = uuid.into();
        let conn = DATABASE.get().unwrap();
        let app = states::app_state().expect("App state not initialized");

        let mut transaction_lookup = TransactionPaginationQueryDto::new(1, -1);

        let (bought_price, list_price) = match StockRivenQuery::get_by_uuid(conn, &uuid).await {
            Ok(item) => {
                if let Some(item) = item {
                    transaction_lookup =
                        transaction_lookup.set_unique_name(&item.weapon_unique_name);
                    (item.bought as i64, item.list_price.unwrap_or(0))
                } else {
                    (-1, 0)
                }
            }
            Err(e) => return Err(e.with_location(get_location!())),
        };

        let potential_profit = match app.wfm_client.auction().cache_auctions().get_by_uuid(&uuid) {
            Some(auction) => {
                transaction_lookup.set_wfm_url(&auction.item.weapon_url_name);
                auction.starting_price as i64 - bought_price
            }
            None => list_price,
        };

        let transaction_paginate = TransactionQuery::get_all(conn, transaction_lookup)
            .await
            .map_err(|e| e.with_location(get_location!()))?;

        Ok(Self {
            report: FinancialReport::from(&transaction_paginate.results),
            last_transactions: transaction_paginate.take_top(5),
            bought_price,
            potential_profit,
        })
    }
    pub async fn try_from_model(item: &stock_riven::Model) -> Result<Self, Error> {
        let mut summary = RivenFinancialSummary::new(&item.weapon_unique_name).await?;
        let app = states::app_state().map_err(|e| e.with_location(get_location!()))?;

        let potential_profit = match app
            .wfm_client
            .auction()
            .cache_auctions()
            .get_by_uuid(&item.uuid)
        {
            Some(auction) => auction.starting_price as i64 - item.bought,
            None => item.list_price.unwrap_or(0),
        };

        summary.potential_profit = potential_profit;
        summary.bought_price = item.bought;

        Ok(summary)
    }
}

impl Default for RivenFinancialSummary {
    fn default() -> Self {
        Self {
            report: FinancialReport::default(),
            last_transactions: vec![],
            bought_price: -1,
            potential_profit: -1,
        }
    }
}
