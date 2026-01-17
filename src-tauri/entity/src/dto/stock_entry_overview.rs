use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(FromQueryResult, Debug, Serialize, Deserialize)]
pub struct StockEntryOverview {
    pub id: String,
    pub key: String,
    #[sea_orm(default_value = 0)]
    pub count: i64,
    #[sea_orm(default_value = 0)]
    pub revenue: i64,
    #[sea_orm(default_value = 0)]
    pub expenses: i64,
    #[sea_orm(default_value = 0)]
    pub profit: i64,
}
