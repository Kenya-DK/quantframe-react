use std::sync::{Arc, Mutex};

use crate::{
    auth::AuthState,
    cache::CacheState,
    database::DatabaseClient,
    helper::{self, ColumnType, ColumnValues},
    logger,
    settings::SettingsState,
    structs::{GlobleError, Invantory, Order, Transaction},
    wfm_client::WFMClientState,
};
use polars::{
    lazy::dsl::col,
    prelude::{DataFrame, NamedFrom},
    series::Series,
};
use serde_json::json;
use sqlx::{migrate::MigrateDatabase, Pool, Row, Sqlite, SqlitePool};

#[derive(Clone, Debug)]
pub struct DebugClient {
    log_file: String,
    cache: Arc<Mutex<CacheState>>,
    wfm: Arc<Mutex<WFMClientState>>,
    auth: Arc<Mutex<AuthState>>,
    db: Arc<Mutex<DatabaseClient>>,
    settings: Arc<Mutex<SettingsState>>,
}

impl DebugClient {
    pub fn new(
        cache: Arc<Mutex<CacheState>>,
        wfm: Arc<Mutex<WFMClientState>>,
        auth: Arc<Mutex<AuthState>>,
        db: Arc<Mutex<DatabaseClient>>,
        settings: Arc<Mutex<SettingsState>>,
    ) -> Self {
        DebugClient {
            log_file: "debug.log".to_string(),
            cache,
            wfm,
            auth,
            db,
            settings,
        }
    }

    // TODO: Remove in production
    pub async fn import_warframe_algo_trader_data(
        &self,
        db_path: String,
        import_type: String,
    ) -> Result<bool, GlobleError> {
        let db = self.db.lock()?.clone();
        let db: Pool<Sqlite> = db.get_connection().clone().lock()?.clone();

        let watdb = SqlitePool::connect(db_path.as_str()).await.unwrap();

        if import_type == "inventory" {
            // Delete all data in the database to prevent duplicates and errors
            sqlx::query("DELETE FROM inventorys;").execute(&db).await?;

            let inventory_vec = sqlx::query("SELECT * FROM inventory;")
                .fetch_all(&watdb)
                .await
                .unwrap();
            for row in inventory_vec {
                let name = row.try_get::<String, _>(1).unwrap();
                let price = row.try_get::<f64, _>(2).unwrap();
                let owned = row.try_get::<i64, _>(4).unwrap();

                let item = self.cache.lock()?.get_item_by_url_name(&name);
                if item.is_none() {
                    logger::error(
                        "Database",
                        format!("Could not find item with name {}", name).as_str(),
                        true,
                        Some(self.log_file.as_str()),
                    );
                    continue;
                }
                let item = item.unwrap();
                let item_id = item.id.clone();
                sqlx::query(
                    "INSERT INTO inventorys (item_id, item_url, item_name, rank, price, owned) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")
                    .bind(item_id.clone())
                    .bind(item.url_name)
                    .bind(item.item_name)
                    .bind(0)
                    .bind(price)
                    .bind(owned)
                    .execute(&db).await?;
            }
        } else if import_type == "transactions" {
            // Delete all data in the database to prevent duplicates and errors
            sqlx::query("DELETE FROM transactions;")
                .execute(&db)
                .await?;
            let transactions_vec = sqlx::query("SELECT * FROM transactions;")
                .fetch_all(&watdb)
                .await
                .unwrap();

            for row in transactions_vec {
                let name = row.try_get::<String, _>(1).unwrap();
                let datetime = row.try_get::<String, _>(2).unwrap();
                let transaction_type = row.try_get::<String, _>(3).unwrap();
                let price = row.try_get::<i64, _>(4).unwrap();

                let item = self.cache.lock()?.get_item_by_url_name(&name);
                if item.is_none() {
                    logger::error(
                        "Database",
                        format!("Could not find item with name {}", name).as_str(),
                        true,
                        Some(self.log_file.as_str()),
                    );
                    continue;
                }
                let item = item.unwrap();
                let item_id = item.id.clone();
                sqlx::query(
                    "INSERT INTO transactions (item_id, item_type, item_url, item_name, datetime, transaction_type, quantity, rank, price) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)")
                    .bind(item_id.clone())
                                .bind(item.tags.unwrap().join(","))
                                .bind(item.url_name)
                                .bind(item.item_name)
                                .bind(datetime)
                                .bind(transaction_type)
                                .bind(1)
                                .bind(0)
                                .bind(price)
                                .execute(&db).await?;
            }
        } else {
            logger::error_con(
                "Debug",
                format!("Could not find import type {}", import_type).as_str(),
            );
        }

        Ok(true)
    }

    pub async fn reset_data(&self, reset_type: String) -> Result<bool, GlobleError> {
        let db = self.db.lock()?.clone();
        let db: Pool<Sqlite> = db.get_connection().clone().lock()?.clone();
        if reset_type == "inventory" {
            // Delete all data in the database to prevent duplicates and errors
            sqlx::query("DELETE FROM inventorys;").execute(&db).await?;
        } else if reset_type == "transactions" {
            // Delete all data in the database to prevent duplicates and errors
            sqlx::query("DELETE FROM transactions;")
                .execute(&db)
                .await?;
        } else {
            logger::error_con(
                "Debug",
                format!("Could not find import type {}", reset_type).as_str(),
            );
        }
        Ok(true)
    }
}
