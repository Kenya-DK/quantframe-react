use crate::{
    auth::AuthState,
    database::client::DBClient,
    error::AppError,
    helper,
    logger::{self, LogLevel},
    structs::{Auction, RivenAttribute},
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
    ListedPrice,
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
    pub listed_price: Option<i32>,
    pub created: String,
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
            .col(ColumnDef::new(StockRiven::Attributes).json())
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
                ColumnDef::new(StockRiven::ListedPrice)
                    .integer()
                    .default(Value::Int(None)),
            )
            .col(ColumnDef::new(StockRiven::Created).date_time().not_null())
            .build(SqliteQueryBuilder);

        sqlx::query(&sql)
            .execute(&connection)
            .await
            .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;
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
                StockRiven::MasteryRank,
                StockRiven::ReRolls,
                StockRiven::Polarity,
                StockRiven::Price,
                StockRiven::ListedPrice,
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
    pub async fn create(
        &self,
        order_id: Option<String>,
        url_name: &str,
        mod_name: &str,
        price: f64,
        rank: i32,
        attributes: Vec<RivenAttribute>,
        mastery_rank: i32,
        re_rolls: i32,
        polarity: &str,
    ) -> Result<StockRivenStruct, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        let cache = self.client.cache.lock().unwrap().clone();
        let item = cache.riven().find_type(url_name)?.unwrap();
        let mut inventory = StockRivenStruct {
            id: 0,
            order_id: order_id.clone(),
            weapon_id: item.id,
            weapon_url: url_name.to_string(),
            weapon_name: item.item_name,
            weapon_type: item.riven_type,
            polarity: polarity.to_string(),
            mod_name: mod_name.to_string(),
            rank: rank as i32,
            attributes: sqlx::types::Json(attributes.clone()),
            mastery_rank,
            re_rolls,
            price: price as f64,
            listed_price: None,
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
                StockRiven::MasteryRank,
                StockRiven::ReRolls,
                StockRiven::Price,
                StockRiven::Polarity,
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
                inventory.mastery_rank.into(),
                inventory.re_rolls.into(),
                inventory.price.into(),
                inventory.polarity.clone().into(),
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
        auction: &Auction<String>,
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
                auction.item.mastery_level.expect("No mastery rank found") as i32,
                auction.item.re_rolls.expect("No re-rolls found") as i32,
                &auction.item.polarity.clone().expect("No polarity found"),
            )
            .await?;

        Ok(riven)
    }
    pub fn emit(&self, operation: &str, data: serde_json::Value) {
        helper::emit_update("StockRivens", operation, Some(data));
    }
}
