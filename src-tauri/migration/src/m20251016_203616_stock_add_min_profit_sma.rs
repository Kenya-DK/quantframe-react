use crate::m20240406_135257_create_stock_item_table::StockItem;

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(StockItem::Table)
                    .add_column(ColumnDef::new(StockItem::MinimumProfit).integer())
                    .to_owned(),
            )
            .await
            .unwrap();
        manager
            .alter_table(
                Table::alter()
                    .table(StockItem::Table)
                    .add_column(ColumnDef::new(StockItem::MinimumSMA).integer())
                    .to_owned(),
            )
            .await
            .unwrap();
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(StockItem::Table)
                    .drop_column(StockItem::MinimumProfit)
                    .drop_column(StockItem::MinimumSMA)
                    .to_owned(),
            )
            .await
    }
}
