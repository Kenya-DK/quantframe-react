use crate::{
    app::client::AppState, cache::client::CacheClient, helper, logger,
    notification::client::NotifyClient, utils::modules::error::AppError,
};
use entity::{
    enums::stock_status::StockStatus,
    price_history::{PriceHistory, PriceHistoryVec},
    stock_item,
    stock_riven::{self, MatchRivenStruct},
    sub_type::SubType,
    transaction::{self, TransactionItemType},
};
use eyre::eyre;

use serde_json::{json, Value};
use service::{
    sea_orm::{Database, DatabaseConnection, EntityTrait},
    StockItemMutation, StockItemQuery, StockRivenMutation, StockRivenQuery, TransactionMutation,
    TransactionQuery,
};
use std::{
    fs,
    io::Read as _,
    path::Path,
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub struct DebugClient {
    log_file: String,
    app: Arc<Mutex<AppState>>,
    cache: Arc<Mutex<CacheClient>>,
    notify: Arc<Mutex<NotifyClient>>,
}

impl DebugClient {
    pub fn new(
        cache: Arc<Mutex<CacheClient>>,
        app: Arc<Mutex<AppState>>,
        notify: Arc<Mutex<NotifyClient>>,
    ) -> Self {
        DebugClient {
            log_file: "debug.log".to_string(),
            cache,
            app,
            notify,
        }
    }

    pub async fn migrate_data_transactions(
        &self,
        old_con: &DatabaseConnection,
        new_con: &DatabaseConnection,
    ) -> Result<(), AppError> {
        let cache = self.cache.lock()?.clone();
        let notify = self.notify.lock()?.clone();
        // Migrate the database transactions
        let old_items = TransactionQuery::get_old_transactions(old_con)
            .await
            .map_err(|e| AppError::new_db("MigrateDataBase", e))?;
        for item in old_items {
            println!("Migrating transaction: {:?}", item.name);

            let item_unique_name = match cache.tradable_items().find_by_url_name(&item.url) {
                Some(item) => item.unique_name,
                None => match cache.riven().find_riven_type_by_url_name(&item.url) {
                    Some(item) => item.unique_name,
                    None => "".to_string(),
                },
            };
            let sub_type = if item.rank > 0 || item.item_type == "riven" {
                Some(SubType {
                    rank: Some(item.rank as i64),
                    variant: None,
                    cyan_stars: None,
                    amber_stars: None,
                })
            } else {
                None
            };

            let transaction_type = match item.transaction_type.as_str() {
                "sell" => transaction::TransactionType::Sale,
                "buy" => transaction::TransactionType::Purchase,
                _ => panic!("Invalid transaction type"),
            };

            TransactionMutation::create_from_old(
                new_con,
                transaction::Model {
                    id: 0,
                    wfm_id: item.wfm_id,
                    wfm_url: item.url,
                    item_name: item.name,
                    item_type: TransactionItemType::from_str(&item.item_type),
                    item_unique_name,
                    sub_type,
                    tags: item.tags,
                    transaction_type,
                    quantity: item.quantity as i64,
                    user_name: "".to_string(),
                    price: item.price as i64,
                    created_at: item.created.parse().unwrap(),
                    updated_at: item.created.parse().unwrap(),
                    properties: item.properties,
                },
            )
            .await
            .unwrap();
        }
        let new_items = TransactionQuery::get_all(new_con)
            .await
            .map_err(|e| AppError::new_db("MigrateDataBase", e))?;
        notify.gui().send_event_update(
            crate::utils::enums::ui_events::UIEvent::UpdateTransaction,
            crate::utils::enums::ui_events::UIOperationEvent::Set,
            Some(json!(new_items)),
        );
        Ok(())
    }

    pub async fn migrate_data_stock_item(
        &self,
        old_con: &DatabaseConnection,
        new_con: &DatabaseConnection,
    ) -> Result<(), AppError> {
        let cache = self.cache.lock()?.clone();
        let notify = self.notify.lock()?.clone();
        let old_items = StockItemQuery::get_old_stock_items(old_con)
            .await
            .map_err(|e| AppError::new_db("MigrateDataBase", e))?;
        for item in old_items {
            println!("Migrating transaction: {:?}", item.name);

            let item_unique_name = match cache.tradable_items().find_by_url_name(&item.url) {
                Some(item) => item.unique_name,
                None => match cache.riven().find_riven_type_by_url_name(&item.url) {
                    Some(item) => item.unique_name,
                    None => "".to_string(),
                },
            };
            let sub_type = if item.rank > 0 {
                Some(SubType {
                    rank: Some(item.rank as i64),
                    variant: None,
                    cyan_stars: None,
                    amber_stars: None,
                })
            } else {
                None
            };

            StockItemMutation::create_from_old(
                new_con,
                stock_item::Model {
                    id: 0,
                    wfm_id: item.wfm_id,
                    wfm_url: item.url,
                    item_name: item.name,
                    item_unique_name,
                    sub_type,
                    bought: item.price as i64,
                    minimum_price: item.minium_price.map(|price| price as i64),
                    list_price: item.listed_price.map(|price| price as i64),
                    owned: item.owned as i64,
                    is_hidden: item.hidden,
                    status: StockStatus::from_string(&item.status),
                    price_history: PriceHistoryVec(vec![]),
                    updated_at: chrono::Utc::now(),
                    created_at: chrono::Utc::now(),
                },
            )
            .await
            .unwrap();
        }
        let new_items = StockItemQuery::get_all(new_con)
            .await
            .map_err(|e| AppError::new_db("MigrateDataBase", e))?;
        notify.gui().send_event_update(
            crate::utils::enums::ui_events::UIEvent::UpdateStockItems,
            crate::utils::enums::ui_events::UIOperationEvent::Set,
            Some(json!(new_items)),
        );
        Ok(())
    }

    pub async fn migrate_data_stock_riven(
        &self,
        old_con: &DatabaseConnection,
        new_con: &DatabaseConnection,
    ) -> Result<(), AppError> {
        let cache = self.cache.lock()?.clone();
        let notify = self.notify.lock()?.clone();
        let old_items = StockRivenQuery::get_old_stock_riven(old_con)
            .await
            .map_err(|e| AppError::new_db("MigrateDataBase", e))?;
        for item in old_items {
            let item_unique_name = match cache.tradable_items().find_by_url_name(&item.weapon_url) {
                Some(item) => item.unique_name,
                None => match cache.riven().find_riven_type_by_url_name(&item.weapon_url) {
                    Some(item) => item.unique_name,
                    None => "".to_string(),
                },
            };
            let sub_type = Some(SubType {
                rank: Some(item.rank as i64),
                variant: None,
                cyan_stars: None,
                amber_stars: None,
            });

            StockRivenMutation::create_from_old(
                new_con,
                stock_riven::Model {
                    id: 0,
                    wfm_order_id: item.order_id,
                    wfm_weapon_id: item.weapon_id,
                    wfm_weapon_url: item.weapon_url,
                    weapon_name: item.weapon_name,
                    weapon_type: item.weapon_type,
                    weapon_unique_name: item_unique_name,
                    sub_type,
                    mod_name: item.mod_name,
                    attributes: item.attributes,
                    mastery_rank: item.mastery_rank as i64,
                    re_rolls: item.re_rolls as i64,
                    polarity: item.polarity,
                    bought: item.price as i64,
                    minimum_price: item.minium_price.map(|price| price as i64),
                    list_price: item.listed_price.map(|price| price as i64),
                    filter: MatchRivenStruct::new(),
                    is_hidden: item.private,
                    status: StockStatus::from_string(&item.status),
                    comment: item.comment.unwrap_or("".to_string()),
                    price_history: PriceHistoryVec(vec![]),
                    updated_at: chrono::Utc::now(),
                    created_at: chrono::Utc::now(),
                },
            )
            .await
            .unwrap();
        }
        let new_items = StockRivenQuery::get_all(new_con)
            .await
            .map_err(|e| AppError::new_db("MigrateDataBase", e))?;
        notify.gui().send_event_update(
            crate::utils::enums::ui_events::UIEvent::UpdateStockRivens,
            crate::utils::enums::ui_events::UIOperationEvent::Set,
            Some(json!(new_items)),
        );
        Ok(())
    }
    pub async fn migrate_data_all(
        &self,
        old_con: &DatabaseConnection,
        new_con: &DatabaseConnection,
    ) -> Result<(), AppError> {
        self.migrate_data_transactions(old_con, new_con).await?;
        self.migrate_data_stock_item(old_con, new_con).await?;
        self.migrate_data_stock_riven(old_con, new_con).await?;
        Ok(())
    }
}
