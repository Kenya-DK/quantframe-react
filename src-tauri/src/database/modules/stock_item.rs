use crate::{
    auth::AuthState,
    database::client::DBClient,
    error::AppError,
    helper,
    logger::{self, LogLevel},
    structs::RivenAttribute,
};
use eyre::eyre;
use polars::{
    prelude::{DataFrame, NamedFrom},
    series::Series,
};
use reqwest::header::HeaderMap;
use sea_query::{ColumnDef, Expr, Iden, InsertStatement, Query, SqliteQueryBuilder, Table, Value};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Row;

#[derive(Iden)]
pub enum Inventory {
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
    Polarity,
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
    pub rank: i32,
    // Used for relics
    pub sub_type: Option<String>,
    // Used for riven mods
    pub attributes: sqlx::types::Json<Vec<RivenAttribute>>,
    // Used for riven mods
    pub mastery_rank: Option<i32>,
    // Used for riven mods
    pub re_rolls: Option<i32>,
    // Used for riven mods
    pub polarity: Option<String>,
    pub price: f64,
    pub listed_price: Option<i32>,
    pub owned: i32,
    pub created: String,
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
            .table(Inventory::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Inventory::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Inventory::ItemId).uuid().not_null())
            .col(ColumnDef::new(Inventory::ItemUrl).string().not_null())
            .col(ColumnDef::new(Inventory::ItemName).string().not_null())
            .col(ColumnDef::new(Inventory::ItemType).string().not_null())
            .col(
                ColumnDef::new(Inventory::Rank)
                    .integer()
                    .not_null()
                    .default(Value::Int(Some(0))),
            )
            .col(ColumnDef::new(Inventory::SubType).string())
            .col(ColumnDef::new(Inventory::Attributes).json())
            .col(ColumnDef::new(Inventory::MasteryRank).integer())
            .col(ColumnDef::new(Inventory::ReRolls).integer())
            .col(ColumnDef::new(Inventory::Polarity).string())
            .col(
                ColumnDef::new(Inventory::Price)
                    .float()
                    .not_null()
                    .default(Value::Int(Some(0))),
            )
            .col(
                ColumnDef::new(Inventory::ListedPrice)
                    .integer()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(Inventory::Owned)
                    .integer()
                    .not_null()
                    .default(Value::Int(Some(1))),
            )
            .col(ColumnDef::new(Inventory::Created).date_time().not_null())
            .build(SqliteQueryBuilder);

        sqlx::query(&sql)
            .execute(&connection)
            .await
            .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;
        Ok(true)
    }

    pub async fn get_items(&self) -> Result<Vec<InventoryStruct>, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        // Read
        let sql = Query::select()
            .columns([
                Inventory::Id,
                Inventory::ItemId,
                Inventory::ItemUrl,
                Inventory::ItemName,
                Inventory::ItemType,
                Inventory::Rank,
                Inventory::SubType,
                Inventory::Attributes,
                Inventory::MasteryRank,
                Inventory::ReRolls,
                Inventory::Polarity,
                Inventory::Price,
                Inventory::ListedPrice,
                Inventory::Owned,
                Inventory::Created,
            ])
            .from(Inventory::Table)
            .to_string(SqliteQueryBuilder);

        let rows = sqlx::query_as::<_, InventoryStruct>(&sql)
            .fetch_all(&connection)
            .await
            .unwrap();
        Ok(rows)
    }

    pub async fn get_item_by_url_name(
        &self,
        url_name: &str,
    ) -> Result<Option<InventoryStruct>, AppError> {
        let inventorys = self.get_items().await?;
        let inventory = inventorys.iter().find(|t| t.item_url == url_name);
        Ok(inventory.cloned())
    }

    pub async fn create(
        &self,
        url_name: &str,
        item_type: &str,
        mut quantity: i32,
        price: f64,
        rank: i32,
        sub_type: Option<&str>,
        attributes: Option<Vec<RivenAttribute>>,
        mastery_rank: Option<i32>,
        re_rolls: Option<i32>,
        polarity: Option<&str>,
    ) -> Result<InventoryStruct, AppError> {
        let inventorys = self.get_item_by_url_name(url_name).await?;
        let connection = self.client.connection.lock().unwrap().clone();

        let attributes = match attributes {
            Some(t) => t,
            None => vec![],
        };

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

                self.update_by_id(t.id, Some(total_owned), Some(weighted_price), None)
                    .await?;
                let mut t = t.clone();
                t.owned = total_owned;
                t.price = weighted_price;
                t
            }
            None => {
                let price = price / (quantity as f64);

                let mut inventory = InventoryStruct {
                    id: 0,
                    item_id: item.clone().id,
                    item_url: item.clone().url_name,
                    item_name: item.clone().item_name,
                    item_type: item_type.to_string(),
                    rank: rank as i32,
                    sub_type: sub_type.map(|t| t.to_string()),
                    attributes: sqlx::types::Json(attributes.clone()),
                    mastery_rank,
                    re_rolls,
                    polarity: polarity.map(|t| t.to_string()),
                    price: price as f64,
                    listed_price: None,
                    owned: quantity as i32,
                    created: chrono::Local::now().naive_local().to_string(),
                };

                let sql = InsertStatement::default()
                    .into_table(Inventory::Table)
                    .columns([
                        Inventory::ItemId,
                        Inventory::ItemUrl,
                        Inventory::ItemName,
                        Inventory::ItemType,
                        Inventory::Rank,
                        Inventory::SubType,
                        Inventory::Attributes,
                        Inventory::MasteryRank,
                        Inventory::ReRolls,
                        Inventory::Polarity,
                        Inventory::Price,
                        Inventory::Owned,
                        Inventory::Created,
                    ])
                    .values_panic([
                        inventory.item_id.clone().into(),
                        inventory.item_url.clone().into(),
                        inventory.item_name.clone().into(),
                        inventory.item_type.clone().into(),
                        inventory.rank.into(),
                        inventory.sub_type.clone().into(),
                        serde_json::to_value(&inventory.attributes).unwrap().into(),
                        inventory.mastery_rank.into(),
                        inventory.re_rolls.into(),
                        inventory.polarity.clone().into(),
                        inventory.price.into(),
                        inventory.owned.into(),
                        inventory.created.clone().into(),
                    ])
                    .to_string(SqliteQueryBuilder);
                let row = sqlx::query(&sql.replace("\\", ""))
                    .execute(&connection)
                    .await
                    .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;
                let id = row.last_insert_rowid();
                inventory.id = id;

                inventory
            }
        };
        // Update UI
        self.emit(
            "CREATE_OR_UPDATE",
            serde_json::to_value(inventory.clone()).unwrap(),
        );
        Ok(inventory)
    }

    pub async fn update_by_id(
        &self,
        id: i64,
        owned: Option<i32>,
        price: Option<f64>,
        listed_price: Option<i32>,
    ) -> Result<InventoryStruct, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        let items = self.get_items().await?;
        let inventory = items.iter().find(|t| t.id == id);
        if inventory.is_none() {
            return Err(AppError::new_with_level(
                "Database",
                eyre!("Item not found in database"),
                LogLevel::Error,
            ));
        }
        let mut inventory = inventory.unwrap().clone();
        let mut values = vec![(Inventory::ListedPrice, listed_price.into())];

        if owned.is_some() {
            inventory.owned = owned.unwrap();
            values.push((Inventory::Owned, owned.into()));
        }

        if price.is_some() {
            inventory.price = price.unwrap();
            values.push((Inventory::Price, price.into()));
        }

        if listed_price.is_some() && listed_price.unwrap() > -1 {
            inventory.listed_price = listed_price;
            values.push((Inventory::ListedPrice, listed_price.into()));
        }

        let sql = Query::update()
            .table(Inventory::Table)
            .values(values)
            .and_where(Expr::col(Inventory::Id).eq(id))
            .to_string(SqliteQueryBuilder);
        sqlx::query(&sql)
            .execute(&connection)
            .await
            .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;

        self.emit(
            "CREATE_OR_UPDATE",
            serde_json::to_value(inventory.clone()).unwrap(),
        );
        Ok(inventory.clone())
    }

    pub async fn update_by_url(
        &self,
        id: &str,
        owned: Option<i32>,
        price: Option<f64>,
        listed_price: Option<i32>,
    ) -> Result<InventoryStruct, AppError> {
        let items = self.get_items().await?;
        let item = items.iter().find(|t| t.item_url == id);
        if item.is_none() {
            return Err(AppError::new_with_level(
                "Database",
                eyre!("Item not found in database: {}", id),
                LogLevel::Warning,
            ));
        }
        let item = item.unwrap();
        self.update_by_id(item.id, owned, price, listed_price)
            .await?;
        Ok(self
            .update_by_id(item.id, owned, price, listed_price)
            .await?)
    }

    pub async fn delete(&self, id: i64) -> Result<InventoryStruct, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        let items = self.get_items().await?;

        let inventory = items.iter().find(|t| t.id == id);
        if inventory.is_none() {
            return Err(AppError::new_with_level(
                "Database",
                eyre!("Item not found in database"),
                LogLevel::Error,
            ));
        }
        let sql = Query::delete()
            .from_table(Inventory::Table)
            .and_where(Expr::col(Inventory::Id).eq(id))
            .to_string(SqliteQueryBuilder);
        sqlx::query(&sql)
            .execute(&connection)
            .await
            .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;
        self.emit(
            "DELETE",
            serde_json::to_value(inventory.unwrap().clone()).unwrap(),
        );
        Ok(inventory.unwrap().clone())
    }

    pub async fn sell_item(
        &self,
        id: i64,
        item_type: &str,
        price: i32,
        mut quantity: i32,
    ) -> Result<InventoryStruct, AppError> {
        let items = self.get_items().await?;
        let inventory = items.iter().find(|t| t.id == id);

        if inventory.is_none() {
            return Err(AppError::new_with_level(
                "Database",
                eyre!("Item not found in database"),
                LogLevel::Error,
            ));
        }

        let mut inventory = inventory.unwrap().clone();
        if quantity <= 0 {
            quantity = 1;
        }
        inventory.owned -= quantity;

        if inventory.owned <= 0 {
            self.delete(id).await?;
        } else {
            self.update_by_id(id, Some(inventory.owned.clone()), None, None)
                .await?;
        }
        Ok(inventory.clone())
    }
    pub async fn get_inventory_names(&self) -> Result<Vec<String>, AppError> {
        let inventorys = self.get_items().await?;
        let names = inventorys
            .iter()
            .map(|t| t.item_url.clone())
            .collect::<Vec<_>>();
        Ok(names)
    }
    pub fn emit(&self, operation: &str, data: serde_json::Value) {
        helper::emit_update("inventorys", operation, Some(data));
    }
    // End of methods
    pub fn convet_inventorys_to_datafream(
        &self,
        inventorys: Vec<InventoryStruct>,
    ) -> Result<DataFrame, AppError> {
        let df = DataFrame::new(vec![
            Series::new("id", inventorys.iter().map(|i| i.id).collect::<Vec<_>>()),
            Series::new(
                "item_id",
                inventorys
                    .iter()
                    .map(|i| i.item_id.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "item_url",
                inventorys
                    .iter()
                    .map(|i| i.item_url.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "item_name",
                inventorys
                    .iter()
                    .map(|i| i.item_name.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "item_type",
                inventorys
                    .iter()
                    .map(|i| i.item_type.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "rank",
                inventorys.iter().map(|i| i.rank).collect::<Vec<_>>(),
            ),
            Series::new(
                "price",
                inventorys.iter().map(|i| i.price).collect::<Vec<_>>(),
            ),
            Series::new(
                "listed_price",
                inventorys
                    .iter()
                    .map(|i| i.listed_price)
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "owned",
                inventorys.iter().map(|i| i.owned).collect::<Vec<_>>(),
            ),
        ]);
        Ok(df.unwrap())
    }
}
