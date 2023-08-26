use std::sync::{Arc, Mutex};

use crate::{
    cache::CacheState,
    helper::{self, ColumnType, ColumnValues},
    logger,
    structs::{GlobleError, Invantory, Transaction},
};
use polars::{
    lazy::dsl::col,
    prelude::{DataFrame, NamedFrom},
    series::Series,
};
use serde_json::json;
use sqlx::{migrate::MigrateDatabase, Pool, Row, Sqlite, SqlitePool};

#[derive(Clone, Debug)]
pub struct DatabaseClient {
    log_file: String,
    connection: Arc<Mutex<Pool<Sqlite>>>,
    cache: Arc<Mutex<CacheState>>,
}

impl DatabaseClient {
    pub async fn new(cache: Arc<Mutex<CacheState>>) -> Result<Self, GlobleError> {
        let mut db_url = helper::get_app_roaming_path();
        db_url.push("quantframe.sqlite");
        let db_url: &str = db_url.to_str().unwrap();
        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
            match Sqlite::create_database(db_url).await {
                Ok(_) => logger::info_con(
                    "Database",
                    format!("Database created at {}", db_url).as_str(),
                ),
                Err(error) => logger::error(
                    "Database",
                    format!("Error creating database: {:?}", error).as_str(),
                    true,
                    None,
                ),
            }
        }
        Ok(DatabaseClient {
            log_file: "db.log".to_string(),
            connection: Arc::new(Mutex::new(SqlitePool::connect(db_url).await.unwrap())),
            cache,
        })
    }
    // Initialize the database
    pub async fn initialize(&self) -> Result<bool, GlobleError> {
        logger::info("Database", "Initialize", true, None);
        let connection = self.connection.lock().unwrap().clone();
        sqlx::query(
            "
        CREATE TABLE IF NOT EXISTS inventorys (
            id integer not null primary key autoincrement,
            item_id text not null,
            item_url text not null,
            item_name text not null,
            rank integer not null default 0,
            price REAL not null default 0,
            listed_price INT default null,
            owned INT not null default 1
        )",
        )
        .execute(&connection)
        .await
        .unwrap();
        sqlx::query(
            "
        CREATE TABLE IF NOT EXISTS transactions (
            id integer not null primary key autoincrement,
            item_id text not null,
            item_type text not null,
            item_url text not null,
            item_name text not null,
            datetime TEXT,
            transaction_type TEXT,
            quantity integer not null default 1,
            rank integer not null default 0,
            price integer not null default 0
        )",
        )
        .execute(&connection)
        .await
        .unwrap();
        Ok(true)
    }
    pub async fn get_inventory_names(&self) -> Result<Vec<String>, GlobleError> {
        let names = match helper::get_column_values(
            self.get_inventorys_df().await?,
            Some(col("owned").gt(0)),
            "item_url",
            ColumnType::String,
        )
        .expect("")
        {
            ColumnValues::String(values) => values,
            _ => return Err(GlobleError::OtherError("Well Shit".to_string())),
        };
        Ok(names)
    }
    pub async fn get_inventorys_df(&self) -> Result<DataFrame, GlobleError> {
        let inventory_vec = self.get_inventorys().await?;

        let df = DataFrame::new(vec![
            Series::new("id", inventory_vec.iter().map(|i| i.id).collect::<Vec<_>>()),
            Series::new(
                "item_id",
                inventory_vec
                    .iter()
                    .map(|i| i.item_id.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "item_url",
                inventory_vec
                    .iter()
                    .map(|i| i.item_url.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "item_name",
                inventory_vec
                    .iter()
                    .map(|i| i.item_name.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "rank",
                inventory_vec.iter().map(|i| i.rank).collect::<Vec<_>>(),
            ),
            Series::new(
                "price",
                inventory_vec.iter().map(|i| i.price).collect::<Vec<_>>(),
            ),
            Series::new(
                "listed_price",
                inventory_vec
                    .iter()
                    .map(|i| i.listed_price)
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "owned",
                inventory_vec.iter().map(|i| i.owned).collect::<Vec<_>>(),
            ),
        ]);
        Ok(df.unwrap())
    }
    pub async fn get_transactions(&self) -> Result<Vec<Transaction>, GlobleError> {
        let connection = self.connection.lock().unwrap().clone();

        let inventory_vec: Vec<Transaction> = sqlx::query("SELECT * FROM transactions;")
            .fetch_all(&connection)
            .await
            .unwrap()
            .into_iter()
            .map(|row| Transaction {
                id: row.get(0),
                item_id: row.get(1),
                item_type: row.get(2),
                item_url: row.get(3),
                item_name: row.get(4),
                datetime: row.get(5),
                transaction_type: row.get(6),
                quantity: row.get(7),
                rank: row.get(8),
                price: row.get(9),
            })
            .collect();
        Ok(inventory_vec)
    }
    pub async fn get_inventorys(&self) -> Result<Vec<Invantory>, GlobleError> {
        let connection = self.connection.lock().unwrap().clone();

        let inventory_vec: Vec<Invantory> = sqlx::query("SELECT * FROM inventorys;")
            .fetch_all(&connection)
            .await
            .unwrap()
            .into_iter()
            .map(|row| Invantory {
                id: row.get(0),
                item_id: row.get(1),
                item_url: row.get(2),
                item_name: row.get(3),
                rank: row.get(4),
                price: row.get(5),
                listed_price: row.get(6),
                owned: row.get(7),
            })
            .collect();
        Ok(inventory_vec)
    }

    pub async fn create_transaction_entry(
        &self,
        item_id: String,
        transaction_type: String,
        quantity: i64,
        rank: i64,
        price: i64,
    ) -> Result<Transaction, GlobleError> {
        let connection = self.connection.lock().unwrap().clone();
        let item = self.cache.lock()?.get_item_by_url_name(&item_id).unwrap();
        let transaction = Transaction {
            id: -1,
            item_id,
            item_type: item.tags.unwrap().join(","),
            item_url: item.url_name,
            item_name: item.item_name,
            datetime: chrono::Local::now().to_string(),
            transaction_type,
            quantity,
            rank,
            price,
        };
        let result = sqlx::query(
            "INSERT INTO transactions (item_id, item_type, item_url, item_name, datetime, transaction_type, quantity, rank, price) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)")
            .bind(transaction.clone().item_id)
            .bind(transaction.clone().item_type)
            .bind(transaction.clone().item_url)
            .bind(transaction.clone().item_name)
            .bind(transaction.clone().datetime)
            .bind(transaction.clone().transaction_type)
            .bind(transaction.clone().quantity)
            .bind(transaction.clone().rank)
            .bind(transaction.clone().price)
            .execute(&connection).await?;

        let transaction = Transaction {
            id: result.last_insert_rowid(),
            ..transaction
        };
        helper::send_message_to_window(
            "update_data",
            Some(json!({ "type": "transactions",
                "operation": "create",
                "data": transaction.clone()
            })),
        );
        logger::info(
            "Database",
            format!("Created transaction entry with id {}", transaction.id).as_str(),
            true,
            Some(self.log_file.as_str()),
        );
        Ok(transaction)
    }

    pub async fn get_inventory_by_url_name(
        &self,
        url_name: String,
    ) -> Result<Option<Invantory>, GlobleError> {
        let inventorys = self.get_inventorys().await?;
        let inventory = inventorys.iter().find(|t| t.item_url == url_name);
        Ok(inventory.cloned())
    }
    pub async fn create_inventory_entry(
        &self,
        id: String,
        quantity: i64,
        price: i64,
        rank: i64,
    ) -> Result<Invantory, GlobleError> {
        let inventorys = self.get_inventory_by_url_name(id.clone()).await?;
        let connection = self.connection.lock().unwrap().clone();
        let operation = match inventorys {
            Some(_) => "update",
            None => "create",
        };

        let item = self.cache.lock()?.get_item_by_url_name(&id).unwrap();
        let inventory = match inventorys {
            Some(t) => {
                let total_owned = t.owned + 1;
                let total_price = (t.price * t.owned as f64) + price as f64;
                let weighted_price = total_price / total_owned as f64;
                sqlx::query("UPDATE inventorys SET owned = ?1, price = ?2 WHERE id = ?3")
                    .bind(total_owned)
                    .bind(weighted_price)
                    .bind(t.id)
                    .execute(&connection)
                    .await?;
                let mut t = t.clone();
                t.owned = total_owned;
                t.price = weighted_price;
                t
            }
            None => {
                let result = sqlx::query(
                    "INSERT INTO inventorys (item_id, item_url, item_name, rank, price, owned) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")
                    .bind(item.clone().id)
                    .bind(item.clone().url_name)
                    .bind(item.clone().item_name)
                    .bind(rank)
                    .bind(price)
                    .bind(1)
                    .execute(&connection).await?;

                let inventory = Invantory {
                    id: result.last_insert_rowid(),
                    item_id: item.clone().id,
                    item_url: item.clone().url_name,
                    item_name: item.clone().item_name,
                    rank: rank as i64,
                    price: price as f64,
                    listed_price: None,
                    owned: 1,
                };
                inventory
            }
        };
        self.create_transaction_entry(id, "buy".to_string(), quantity, rank, price)
            .await?;
        helper::send_message_to_window(
            "update_data",
            Some(json!({ "type": "inventorys",
                "operation": operation,
                "data": inventory.clone()
            })),
        );
        logger::info(
            "Database",
            format!("Created inventory entry with id {}", inventory.id).as_str(),
            true,
            Some(self.log_file.as_str()),
        );
        Ok(inventory)
    }
    pub async fn sell_invantory_entry(
        &self,
        id: i64,
        price: i64,
    ) -> Result<Invantory, GlobleError> {
        let inventorys = self.get_inventorys().await?;
        let inventory = inventorys.iter().find(|t| t.id == id).clone();
        if inventory.is_none() {
            return Err(GlobleError::OtherError(
                "Could not find inventory entry".to_string(),
            ));
        }
        let connection = self.connection.lock().unwrap().clone();
        let mut inventory = inventory.unwrap().clone();
        inventory.owned -= 1;
        inventory.price = price as f64;
        self.create_transaction_entry(inventory.clone().item_url, "sell".to_string(), 1, 0, price)
            .await?;
        if inventory.owned <= 0 {
            self.delete_inventory_entry(id).await?;
        } else {
            sqlx::query("UPDATE inventorys SET owned = ?1 WHERE id = ?2")
                .bind(inventory.clone().owned)
                .bind(inventory.clone().id)
                .execute(&connection)
                .await?;
            helper::send_message_to_window(
                "update_data",
                Some(json!({ "type": "inventorys",
                    "operation": "update",
                    "data": inventory.clone()
                })),
            );
        }
        Ok(inventory.clone())
    }
    pub async fn delete_inventory_entry(&self, id: i64) -> Result<Option<Invantory>, GlobleError> {
        let inventorys = self.get_inventorys().await?;
        let inventory = inventorys.iter().find(|t| t.id == id).clone();
        if inventory.is_none() {
            return Ok(None);
        }
        let connection = self.connection.lock().unwrap().clone();
        let result = sqlx::query("DELETE FROM inventorys WHERE id = ?1")
            .bind(id)
            .execute(&connection)
            .await?;

        helper::send_message_to_window(
            "update_data",
            Some(json!({ "type": "inventorys",
                "operation": "delete",
                "data": inventory
            })),
        );
        logger::info(
            "Database",
            format!("Deleted inventory entry with id {}", id).as_str(),
            true,
            Some(self.log_file.as_str()),
        );
        Ok(Some(inventory.unwrap().clone()))
    }
}
