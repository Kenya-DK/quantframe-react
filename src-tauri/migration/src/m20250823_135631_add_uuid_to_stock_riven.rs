use crate::m20240406_135307_create_stock_riven_table::StockRiven;

use entity::stock_riven::*;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ActiveModelTrait, EntityTrait, Set, Statement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(StockRiven::Table)
                    .add_column(
                        ColumnDef::new(StockRiven::UUID)
                            .uuid()
                            .not_null()
                            .default("00000000-0000-0000-0000-000000000000"),
                    )
                    .to_owned(),
            )
            .await
            .ok();
        // Remove WFMOrderId column
        if let Err(err) = manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "ALTER TABLE stock_riven DROP COLUMN wfm_order_id;".to_owned(),
            ))
            .await
        {
            if !err.to_string().contains("no such column") {
                return Err(err);
            }
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(StockRiven::Table)
                    .drop_column(StockRiven::UUID)
                    .to_owned(),
            )
            .await
    }
}
