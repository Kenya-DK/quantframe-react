use sea_orm_migration::prelude::*;

use crate::m20240406_135257_create_stock_item_table::StockItem;
use crate::m20240406_135307_create_stock_riven_table::StockRiven;
use crate::m20240406_135323_create_wish_list_table::WishList;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(StockItem::Table)
                    .add_column(ColumnDef::new(StockItem::Properties).json().default("{}"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(StockRiven::Table)
                    .add_column(
                        ColumnDef::new(StockRiven::Properties)
                            .json()
                            .not_null()
                            .default("{}"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(WishList::Table)
                    .add_column(
                        ColumnDef::new(WishList::Properties)
                            .json()
                            .not_null()
                            .default("{}"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(StockRiven::Table)
                    .drop_column(StockRiven::Properties)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(StockItem::Table)
                    .drop_column(StockItem::Properties)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(WishList::Table)
                    .drop_column(WishList::Properties)
                    .to_owned(),
            )
            .await
    }
}
