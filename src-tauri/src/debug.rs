use crate::{
    auth::AuthState,
    cache::CacheState,
    database::{
        client::DBClient,
        modules::transaction::{Transaction, TransactionStruct},
    },
    error::AppError,
    logger,
    settings::SettingsState,
    structs::Ordres,
    wfm_client::client::WFMClient,
};
use eyre::eyre;
use sea_query::{InsertStatement, SqliteQueryBuilder};
use serde_json::Value;
use sqlx::{Pool, Row, Sqlite, SqlitePool};
use std::{
    fs::File,
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub struct DebugClient {
    log_file: String,
    cache: Arc<Mutex<CacheState>>,
    wfm: Arc<Mutex<WFMClient>>,
    auth: Arc<Mutex<AuthState>>,
    db: Arc<Mutex<DBClient>>,
    settings: Arc<Mutex<SettingsState>>,
}

impl DebugClient {
    pub fn new(
        cache: Arc<Mutex<CacheState>>,
        wfm: Arc<Mutex<WFMClient>>,
        auth: Arc<Mutex<AuthState>>,
        db: Arc<Mutex<DBClient>>,
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
    ) -> Result<bool, AppError> {
        let db = self.db.lock()?.clone();
        let dbcon: Pool<Sqlite> = db.get_connection().clone().lock()?.clone();

        let watdb = SqlitePool::connect(db_path.as_str()).await.unwrap();

        if import_type == "inventory" {
            // Delete all data in the database to prevent duplicates and errors
            sqlx::query("DELETE FROM 'inventory'")
                .execute(&dbcon)
                .await
                .map_err(|e| AppError::new("Debug", eyre!(e.to_string())))?;

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
                db.inventory()
                    .create(&item.url_name, "item", owned as i32, price, 0, None, None, None, None,None)
                    .await?;
            }
        } else if import_type == "transactions" {
            // Delete all data in the database to prevent duplicates and errors
            sqlx::query("DELETE FROM 'transaction'")
                .execute(&dbcon)
                .await
                .map_err(|e| AppError::new("Debug", eyre!(e.to_string())))?;
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
                let sql = InsertStatement::default()
                    .into_table(Transaction::Table)
                    .columns([
                        Transaction::ItemId,
                        Transaction::ItemUrl,
                        Transaction::ItemName,
                        Transaction::ItemType,
                        Transaction::ItemTags,
                        Transaction::Rank,
                        Transaction::Price,
                        Transaction::Attributes,
                        Transaction::TransactionType,
                        Transaction::Quantity,
                        Transaction::Created,
                    ])
                    .values_panic([
                        item.id.clone().into(),
                        item.url_name.clone().into(),
                        item.item_name.clone().into(),
                        "item".into(),
                        item.tags.unwrap().join(",").clone().into(),
                        0.into(),
                        price.into(),
                        "[]".into(),
                        transaction_type.clone().into(),
                        1.into(),
                        datetime.into(),
                    ])
                    .to_string(SqliteQueryBuilder);
                sqlx::query(&sql.replace("\\", ""))
                    .execute(&dbcon)
                    .await
                    .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;
            }
        } else {
            logger::error_con(
                "Debug",
                format!("Could not find import type {}", import_type).as_str(),
            );
        }

        Ok(true)
    }

    pub async fn reset_data(&self, reset_type: String) -> Result<bool, AppError> {
        let db = self.db.lock()?.clone();
        let db: Pool<Sqlite> = db.get_connection().clone().lock()?.clone();
        if reset_type == "inventory" {
            // Delete all data in the database to prevent duplicates and errors
            sqlx::query("DELETE FROM inventory;")
                .execute(&db)
                .await
                .map_err(|e| AppError::new("Debug", eyre!(e.to_string())))?;
        } else if reset_type == "transactions" {
            // Delete all data in the database to prevent duplicates and errors
            sqlx::query("DELETE FROM transaction;")
                .execute(&db)
                .await
                .map_err(|e| AppError::new("Debug", eyre!(e.to_string())))?;
        } else {
            logger::error_con(
                "Debug",
                format!("Could not find import type {}", reset_type).as_str(),
            );
        }
        Ok(true)
    }
}
