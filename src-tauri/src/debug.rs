use crate::{
    app::client::AppState, cache::client::CacheClient, helper, logger,
    utils::modules::error::AppError,
};
use entity::transaction;
use eyre::eyre;

use serde_json::{json, Value};
use service::{sea_orm::Database, TransactionMutation, TransactionQuery};
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
}

impl DebugClient {
    pub fn new(cache: Arc<Mutex<CacheClient>>, app: Arc<Mutex<AppState>>) -> Self {
        DebugClient {
            log_file: "debug.log".to_string(),
            cache,
            app,
        }
    }

    pub async fn migrate_data_base(&self) -> Result<(), AppError> {
        let app = self.app.lock()?.clone();
        let cache = self.cache.lock()?.clone();

        // Check if the old database exists

        let storage_path = helper::get_app_storage_path();

        let db_url = format!(
            "sqlite://{}/{}",
            storage_path.to_str().unwrap(),
            "quantframe.sqlite?mode=rwc"
        );

        let conn = Database::connect(db_url)
            .await
            .expect("Database connection failed");

        // Migrate the database transactions
        let old_transactions = TransactionQuery::get_old_transactions(&conn)
            .await
            .map_err(|e| AppError::new_db("MigrateDataBase", e))?;
        for old_transaction in old_transactions {
            println!("Migrating transaction: {:?}", old_transaction.item_type);

            let item_unique_name = match cache.tradable_items().find_by_url_name(&old_transaction.url) {
                Some(item) => item.unique_name,
                None => match cache.riven().find_riven_type_by_url_name(&old_transaction.url) {
                    Some(item) => item.unique_name,
                    None => "".to_string(),
                },
            };
            let sub_type = if old_transaction.rank > 0 || old_transaction.item_type == "riven"{
                Some(json!({
                    "rank": old_transaction.rank,
                }))
            } else {
                None
            };


            TransactionMutation::create_from_old(
                &app.conn,
                transaction::Model {
                    id: 0,
                    wfm_id: old_transaction.wfm_id,
                    wfm_url: old_transaction.url,
                    item_name: old_transaction.name,
                    item_type: old_transaction.item_type,
                    item_unique_name,
                    sub_type,
                    tags: old_transaction.tags,
                    transaction_type: transaction::TransactionType::from_str(
                        &old_transaction.transaction_type,
                    ),
                    quantity: old_transaction.quantity as i64,
                    user_name: "".to_string(),
                    price: old_transaction.price as i64,
                    created_at: old_transaction.created.parse().unwrap(),
                    updated_at: old_transaction.created.parse().unwrap(),
                    properties: old_transaction.properties,
                },
            )
            .await
            .unwrap();
        }
        Ok(())
    }
}
