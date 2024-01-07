use crate::{
    auth::AuthState,
    database::client::DBClient,
    enums::LogLevel,
    error::AppError,
    helper,
    logger::{self},
    structs::{Auction, RivenAttribute},
};
use eyre::eyre;
use polars::{
    prelude::{DataFrame, NamedFrom},
    series::Series,
};
use reqwest::header::HeaderMap;
use sea_query::{
    Alias, ColumnDef, Expr, Iden, InsertStatement, Query, SqliteQueryBuilder, Table, Value,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Pool, Row, Sqlite};

use super::stock_item::StockItemStruct;

#[derive(Iden)]
pub enum StockRiven {
    Table,
    Id,
    OrderId,
    WeaponId,
    WeaponUrl,
    WeaponName,
    WeaponType,
    ModName,
    Rank,
    Attributes,
    MasteryRank,
    ReRolls,
    Polarity,
    Price,
    MiniumPrice,
    ListedPrice,
    MatchRiven,
    Private,
    Status,
    Created,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct StockRivenStruct {
    pub id: i64,
    pub order_id: Option<String>,
    pub weapon_id: String,
    pub weapon_url: String,
    pub weapon_name: String,
    pub weapon_type: String,
    pub mod_name: String,
    pub rank: i32,
    pub attributes: sqlx::types::Json<Vec<RivenAttribute>>,
    pub mastery_rank: i32,
    pub re_rolls: i32,
    pub polarity: String,
    pub price: f64,
    pub minium_price: Option<i32>,
    pub listed_price: Option<i32>,
    pub match_riven: sqlx::types::Json<MatchRivenStruct>,
    pub private: bool,
    pub status: String,
    pub created: String,
}
#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct MatchRivenStruct {
    pub rank: Option<MinMaxStruct>,
    pub mastery_rank: Option<MinMaxStruct>,
    pub re_rolls: Option<MinMaxStruct>,
    pub polarity: Option<String>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct MinMaxStruct {
    pub min: i64,
    pub max: i64,
}

pub struct StockRivenModule<'a> {
    pub client: &'a DBClient,
}

impl<'a> StockRivenModule<'a> {
    // Methods sea-query

    // Initialize the database
    pub async fn initialize(&self) -> Result<bool, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        let sql = Table::create()
            .table(StockRiven::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(StockRiven::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(StockRiven::OrderId).uuid())
            .col(ColumnDef::new(StockRiven::WeaponId).uuid().not_null())
            .col(ColumnDef::new(StockRiven::WeaponUrl).string().not_null())
            .col(ColumnDef::new(StockRiven::WeaponName).string().not_null())
            .col(ColumnDef::new(StockRiven::WeaponType).string().not_null())
            .col(ColumnDef::new(StockRiven::ModName).string().not_null())
            .col(
                ColumnDef::new(StockRiven::Rank)
                    .integer()
                    .not_null()
                    .default(Value::Int(Some(0))),
            )
            .col(
                ColumnDef::new(StockRiven::Attributes)
                    .json()
                    .not_null()
                    .default(json!([])),
            )
            .col(
                ColumnDef::new(StockRiven::MatchRiven)
                    .json()
                    .not_null()
                    .default(json!({})),
            )
            .col(
                ColumnDef::new(StockRiven::MasteryRank)
                    .integer()
                    .not_null()
                    .default(Value::Int(Some(0))),
            )
            .col(
                ColumnDef::new(StockRiven::ReRolls)
                    .integer()
                    .not_null()
                    .default(Value::Int(Some(0))),
            )
            .col(ColumnDef::new(StockRiven::Polarity).string().not_null())
            .col(
                ColumnDef::new(StockRiven::Price)
                    .float()
                    .not_null()
                    .default(Value::Int(Some(0))),
            )
            .col(
                ColumnDef::new(StockRiven::MiniumPrice)
                    .integer()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(StockRiven::ListedPrice)
                    .integer()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(StockRiven::Private)
                    .boolean()
                    .default(Value::Bool(Some(false))),
            )
            .col(
                ColumnDef::new(StockRiven::Status)
                    .string()
                    .not_null()
                    .default("pending"),
            )
            .col(ColumnDef::new(StockRiven::Created).date_time().not_null())
            .build(SqliteQueryBuilder);

        sqlx::query(&sql)
            .execute(&connection)
            .await
            .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;

        // Alter Table
        let mut table = Table::alter()
            .table(StockRiven::Table)
            .add_column(
                ColumnDef::new(StockRiven::MatchRiven)
                    .json()
                    .not_null()
                    .default(json!({})),
            )
            .to_string(SqliteQueryBuilder);
        helper::alter_table(connection.clone(), &table).await?;

        table = Table::alter()
            .table(StockRiven::Table)
            .add_column(
                ColumnDef::new(StockRiven::MiniumPrice)
                    .integer()
                    .default(Value::Int(None)),
            )
            .to_string(SqliteQueryBuilder);
        helper::alter_table(connection.clone(), &table).await?;

        table = Table::alter()
            .table(StockRiven::Table)
            .add_column(
                ColumnDef::new(StockRiven::Status)
                    .string()
                    .not_null()
                    .default("pending"),
            )
            .to_string(SqliteQueryBuilder);
        helper::alter_table(connection.clone(), &table).await?;

        Ok(true)
    }

    pub async fn get_rivens(&self) -> Result<Vec<StockRivenStruct>, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        // Read
        let sql = Query::select()
            .columns([
                StockRiven::Id,
                StockRiven::OrderId,
                StockRiven::WeaponId,
                StockRiven::WeaponUrl,
                StockRiven::WeaponName,
                StockRiven::WeaponType,
                StockRiven::ModName,
                StockRiven::Rank,
                StockRiven::Attributes,
                StockRiven::MatchRiven,
                StockRiven::MasteryRank,
                StockRiven::ReRolls,
                StockRiven::Polarity,
                StockRiven::Price,
                StockRiven::MiniumPrice,
                StockRiven::ListedPrice,
                StockRiven::Private,
                StockRiven::Status,
                StockRiven::Created,
            ])
            .from(StockRiven::Table)
            .to_string(SqliteQueryBuilder);
        let rows = sqlx::query_as::<_, StockRivenStruct>(&sql)
            .fetch_all(&connection)
            .await
            .unwrap();
        Ok(rows)
    }
    pub async fn get_by_id(&self, id: i64) -> Result<Option<StockRivenStruct>, AppError> {
        let stock = self.get_rivens().await?;
        let stock_riven = stock.iter().find(|t| t.id == id);
        Ok(stock_riven.cloned())
    }
    pub async fn create(
        &self,
        order_id: Option<String>,
        url_name: &str,
        mod_name: &str,
        price: f64,
        rank: i32,
        attributes: Vec<RivenAttribute>,
        match_riven: Option<MatchRivenStruct>,
        mastery_rank: i32,
        re_rolls: i32,
        polarity: &str,
        minium_price: Option<i32>,
    ) -> Result<StockRivenStruct, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        let cache = self.client.cache.lock().unwrap().clone();

        let item = match cache.riven().find_type(url_name)? {
            Some(item) => item,
            None => {
                return Err(AppError::new_with_level(
                    "Database",
                    eyre!("Could not find riven in cache: {}", url_name),
                    LogLevel::Critical,
                ))
            }
        };

        let match_riven = match match_riven {
            Some(m) => m,
            None => MatchRivenStruct {
                rank: None,
                mastery_rank: None,
                re_rolls: None,
                polarity: None,
            },
        };

        let mut inventory = StockRivenStruct {
            id: 0,
            order_id: order_id.clone(),
            weapon_id: item.id,
            weapon_url: url_name.to_string(),
            weapon_name: item.item_name,
            weapon_type: item.riven_type.unwrap_or("Unknown".to_string()),
            polarity: polarity.to_string(),
            mod_name: mod_name.to_string(),
            rank: rank as i32,
            attributes: sqlx::types::Json(attributes.clone()),
            match_riven: sqlx::types::Json(match_riven.clone()),
            mastery_rank,
            minium_price,
            re_rolls,
            price: price as f64,
            listed_price: None,
            private: false,
            status: "pending".to_string(),
            created: chrono::Local::now().naive_local().to_string(),
        };

        let sql = InsertStatement::default()
            .into_table(StockRiven::Table)
            .columns([
                StockRiven::OrderId,
                StockRiven::WeaponId,
                StockRiven::WeaponUrl,
                StockRiven::WeaponName,
                StockRiven::WeaponType,
                StockRiven::ModName,
                StockRiven::Rank,
                StockRiven::Attributes,
                StockRiven::MatchRiven,
                StockRiven::MasteryRank,
                StockRiven::ReRolls,
                StockRiven::Price,
                StockRiven::MiniumPrice,
                StockRiven::Polarity,
                StockRiven::Status,
                StockRiven::Created,
            ])
            .values_panic([
                inventory.order_id.clone().into(),
                inventory.weapon_id.clone().into(),
                inventory.weapon_url.clone().into(),
                inventory.weapon_name.clone().into(),
                inventory.weapon_type.clone().into(),
                inventory.mod_name.clone().into(),
                inventory.rank.clone().into(),
                serde_json::to_value(&inventory.attributes).unwrap().into(),
                serde_json::to_value(&inventory.match_riven).unwrap().into(),
                inventory.mastery_rank.into(),
                inventory.re_rolls.into(),
                inventory.price.into(),
                inventory.minium_price.into(),
                inventory.polarity.clone().into(),
                inventory.status.clone().into(),
                inventory.created.clone().into(),
            ])
            .to_string(SqliteQueryBuilder);
        let row = sqlx::query(&sql.replace("\\", ""))
            .execute(&connection)
            .await
            .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;
        let id = row.last_insert_rowid();
        inventory.id = id;

        // Update UI
        self.emit(
            "CREATE_OR_UPDATE",
            serde_json::to_value(inventory.clone()).unwrap(),
        );
        Ok(inventory)
    }

    pub async fn import_auction(
        &self,
        auction: Auction<String>,
        price: i32,
    ) -> Result<StockRivenStruct, AppError> {
        let riven = self
            .create(
                Some(auction.id.clone()),
                &auction.item.weapon_url_name.clone().expect("No Weapon"),
                &auction.item.name.clone().expect("No Name"),
                price as f64,
                auction.item.mod_rank.clone().expect("No rank found") as i32,
                auction
                    .item
                    .attributes
                    .clone()
                    .expect("No attributes found"),
                None,
                auction.item.mastery_level.expect("No mastery rank found") as i32,
                auction.item.re_rolls.expect("No re-rolls found") as i32,
                &auction.item.polarity.clone().expect("No polarity found"),
                None,
            )
            .await?;

        Ok(riven)
    }

    pub async fn reset_listed_price(&self) -> Result<(), AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        let sql = Query::update()
            .table(StockRiven::Table)
            .values([
                (StockRiven::ListedPrice, Value::Int(None)),
                (StockRiven::Status, "pending".into()),
            ])
            .to_string(SqliteQueryBuilder);
        sqlx::query(&sql.replace("\\", ""))
            .execute(&connection)
            .await
            .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;

        self.emit("SET", json!(self.get_rivens().await?));
        Ok(())
    }
    pub async fn update_by_id(
        &self,
        id: i64,
        order_id: Option<String>,
        price: Option<f64>,
        listed_price: Option<i32>,
        visibility: Option<bool>,
        attributes: Option<Vec<RivenAttribute>>,
        match_riven: Option<MatchRivenStruct>,
        minium_price: Option<i32>,
        status: Option<String>,
        private: Option<bool>,
    ) -> Result<StockRivenStruct, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        let items = self.get_rivens().await?;
        let stock_riven = items.iter().find(|t| t.id == id);
        if stock_riven.is_none() {
            return Err(AppError::new_with_level(
                "Database",
                eyre!("Riven not found in database"),
                LogLevel::Error,
            ));
        }
        let mut stock_riven = stock_riven.unwrap().clone();
        let mut values = vec![(StockRiven::ListedPrice, listed_price.into())];

        if order_id.is_some() {
            if order_id.clone().unwrap() == "".to_string()
                || order_id.clone().unwrap() == "null".to_string()
            {
                stock_riven.order_id = None;
            } else {
                stock_riven.order_id = order_id.clone();
            }
            values.push((StockRiven::OrderId, order_id.into()));
        }

        if price.is_some() {
            stock_riven.price = price.unwrap();
            values.push((StockRiven::Price, price.into()));
        }

        if minium_price.is_some() {
            // If minium_price is -1, set it to None
            let minium_price = if minium_price.unwrap() == -1 {
                None
            } else {
                minium_price
            };
            stock_riven.minium_price = minium_price;
            values.push((StockRiven::MiniumPrice, minium_price.into()));
        }

        if listed_price.is_some() && listed_price.unwrap() > -1 {
            stock_riven.listed_price = listed_price;
            values.push((StockRiven::ListedPrice, listed_price.into()));
        }

        if visibility.is_some() {
            stock_riven.private = !visibility.unwrap();
            values.push((StockRiven::Private, stock_riven.private.into()));
        }

        if status.is_some() {
            stock_riven.status = status.unwrap();
            values.push((StockRiven::Status, stock_riven.status.clone().into()));
        }
        if private.is_some() {
            stock_riven.private = private.unwrap();
            values.push((StockRiven::Private, stock_riven.private.clone().into()));
        }

        if attributes.is_some() {
            stock_riven.attributes = sqlx::types::Json(attributes.unwrap());
            values.push((
                StockRiven::Attributes,
                serde_json::to_value(sqlx::types::Json(&stock_riven.attributes.clone()))
                    .unwrap()
                    .into(),
            ));
        }

        if match_riven.is_some() {
            stock_riven.match_riven = sqlx::types::Json(match_riven.unwrap());
            values.push((
                StockRiven::MatchRiven,
                serde_json::to_value(sqlx::types::Json(&stock_riven.match_riven.clone()))
                    .unwrap()
                    .into(),
            ));
        }

        let sql = Query::update()
            .table(StockRiven::Table)
            .values(values)
            .and_where(Expr::col(StockRiven::Id).eq(id))
            .to_string(SqliteQueryBuilder);
        sqlx::query(&sql.replace("\\", ""))
            .execute(&connection)
            .await
            .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;

        self.emit(
            "CREATE_OR_UPDATE",
            serde_json::to_value(stock_riven.clone()).unwrap(),
        );
        Ok(stock_riven.clone())
    }
    pub async fn delete(&self, id: i64) -> Result<StockRivenStruct, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        let items = self.get_rivens().await?;

        let stock_item = items.iter().find(|t| t.id == id);
        if stock_item.is_none() {
            return Err(AppError::new_with_level(
                "Database",
                eyre!("Stock Riven not found in database"),
                LogLevel::Error,
            ));
        }
        let sql = Query::delete()
            .from_table(StockRiven::Table)
            .and_where(Expr::col(StockRiven::Id).eq(id))
            .to_string(SqliteQueryBuilder);
        sqlx::query(&sql)
            .execute(&connection)
            .await
            .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;
        self.emit(
            "DELETE",
            serde_json::to_value(stock_item.unwrap().clone()).unwrap(),
        );
        Ok(stock_item.unwrap().clone())
    }
    pub fn emit(&self, operation: &str, data: serde_json::Value) {
        helper::emit_update("StockRivens", operation, Some(data));
    }
}
