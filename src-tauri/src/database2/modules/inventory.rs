use crate::{
    auth::AuthState, database2::client::DBClient, error::AppError, logger, structs::Invantory,
};
use eyre::eyre;
use reqwest::header::HeaderMap;
use serde_json::json;
use sqlx::Row;

#[derive(Iden)]
enum Inventory {
    Table,
    Id,
    ItemId,
    ItemUrl,
    ItemName,
    ItemType,
    Rank,
    SubType,
    Attributes,
    MasteryRank,
    ReRolls,
    Price,
    ListedPrice,
    Owned,
    Created,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct InventoryStruct {
    pub id: i64,
    pub item_id: String,
    pub item_url: String,
    pub item_name: String,
    pub item_type: String,
    pub rank: i64,
    // Used for relics
    pub sub_type: String,
    // Used for riven mods
    pub attributes: String,
    // Used for riven mods
    pub mastery_rank: i64,
    // Used for riven mods
    pub re_rolls: i64,
    pub price: f64,
    pub listed_price: Option<i64>,
    pub owned: i64,
}

pub struct InventoryModule<'a> {
    pub client: &'a DBClient,
}

impl<'a> InventoryModule<'a> {
    // Methods sea-query

    // Initialize the database
    pub async fn initialize(&self) -> Result<bool, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        let sql = Table::create()
        .table(InventoryStruct::Table)
        .if_not_exists()
        .col(ColumnDef::new(InventoryStruct::Id).integer().not_null().auto_increment().primary_key())
        .col(ColumnDef::new(Inventory::ItemId).uuid().not_null())
        .col(ColumnDef::new(Inventory::ItemUrl).string().not_null())
        .col(ColumnDef::new(Inventory::ItemName).string().not_null())
        .col(ColumnDef::new(Inventory::ItemType).string().not_null())
        .col(ColumnDef::new(Inventory::Rank).integer().not_null().default(Value::Int(0)))
        .col(ColumnDef::new(Inventory::SubType).string())
        .col(ColumnDef::new(Inventory::Attributes).json())
        .col(ColumnDef::new(Inventory::MasteryRank).integer().not_null().default(Value::Int(0)))
        .col(ColumnDef::new(Inventory::ReRolls).integer().not_null().default(Value::Int(0)))
        .col(ColumnDef::new(Inventory::Price).float().not_null().default(Value::Int(0)))
        .col(ColumnDef::new(Inventory::ListedPrice).float().default(Value::Int(None)))
        .col(ColumnDef::new(Inventory::Owned).integer().not_null().default(Value::Int(1)))
        .col(ColumnDef::new(Inventory::Created).date_time().not_null())
        .build(SqliteQueryBuilder);

        let result = sqlx::query(&sql).execute(&connection).await;
        Ok(true)
    }
        
    pub async fn get_items(&self, sql: &str) -> Result<Vec<Invantory>, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();

        let inventory_vec: Vec<Invantory> = sqlx::query(sql)
            .fetch_all(&connection)
            .await
            .unwrap()
            .into_iter()
            .map(|row| Invantory {
                id: row.get(0),
                item_id: row.get(1),
                item_url: row.get(2),
                item_name: row.get(3),
                item_type: row.get(4),
                rank: row.get(5),
                price: row.get(6),
                listed_price: row.get(7),
                owned: row.get(8),
            })
            .collect();
        Ok(inventory_vec)
    }

    pub async fn get_item_by_url_name(
        &self,
        url_name: &str,
    ) -> Result<Option<Invantory>, AppError> {
        let inventorys = self.get_items("SELECT * FROM inventorys;").await?;
        let inventory = inventorys.iter().find(|t| t.item_url == url_name);
        Ok(inventory.cloned())
    }

    pub async fn create(
        &self,
        url_name: String,
        mut quantity: i64,
        price: i64,
        rank: i64,
    ) -> Result<Invantory, AppError> {
        let inventorys = self.get_item_by_url_name(url_name.as_str()).await?;
        let connection = self.client.connection.lock().unwrap().clone();
        let wfm = self.client.wfm.lock()?.clone();

        if quantity <= 0 {
            quantity = 1;
        }

        let item = self
            .client
            .cache
            .lock()?
            .get_item_by_url_name(&url_name)
            .unwrap();
        let inventory = match inventorys {
            Some(t) => {
                let total_owned = t.owned + quantity;
                // Get price per unit
                let total_price = (t.price * t.owned as f64) + price as f64;
                let weighted_price = total_price / total_owned as f64;
                self.update(t.id, total_owned, weighted_price, None)
                    .await?;               
                let mut t = t.clone();
                t.owned = total_owned;
                t.price = weighted_price;
                t
            }
            None => {
                let price = price / quantity;
                let result = sqlx::query(
                        "INSERT INTO inventorys (item_id, item_url, item_name,item_type, rank, price, owned) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")
                        .bind(item.clone().id)
                        .bind(item.clone().url_name)
                        .bind(item.clone().item_name)
                        .bind("item")
                        .bind(rank)
                        .bind(price)
                        .bind(quantity)
                        .execute(&connection).await.map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;

                let inventory = Invantory {
                    id: result.last_insert_rowid(),
                    item_id: item.clone().id,
                    item_url: item.clone().url_name,
                    item_name: item.clone().item_name,
                    item_type: "item".to_string(),
                    rank: rank as i64,
                    price: price as f64,
                    listed_price: None,
                    owned: quantity,
                };
                inventory
            }
        };
        Ok(inventory)
    }

    pub async fn update_by_id(
        &self,
        id: i64,
        owned: Option<i64>,
        price: Option<f64>,
        listed_price: Option<i64>,
    ) -> Result<(), AppError> {
        Ok(())
    }
    
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {        
        Ok(())
    }

    pub fn emit(&'static str, operation: &'static str, data: Value) {
        helper::emit_update("inventorys", operation, Some(data));
    }
    // End of methods
}
