use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(StockItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StockItem::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(StockItem::WFMId).uuid().not_null())
                    .col(ColumnDef::new(StockItem::WFMUrl).string().not_null())
                    .col(ColumnDef::new(StockItem::ItemName).string().not_null())
                    .col(
                        ColumnDef::new(StockItem::ItemUniqueName)
                            .string()
                            .not_null()
                            .default("N/A"),
                    )
                    .col(ColumnDef::new(StockItem::SubType).json())
                    .col(
                        ColumnDef::new(StockItem::Bought)
                            .integer()
                            .not_null()
                            .default(Value::Int(Some(0))),
                    )
                    .col(ColumnDef::new(StockItem::MinimumPrice).integer())
                    .col(ColumnDef::new(StockItem::ListPrice).integer())
                    .col(
                        ColumnDef::new(StockItem::Owned)
                            .integer()
                            .not_null()
                            .default(Value::Int(Some(0))),
                    )
                    .col(
                        ColumnDef::new(StockItem::IsHidden)
                            .boolean()
                            .not_null()
                            .default(Value::Bool(Some(false))),
                    )
                    .col(ColumnDef::new(StockItem::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(StockItem::UpdatedAt).date_time().not_null())
                    .col(
                        ColumnDef::new(StockItem::Status)
                            .string()
                            .not_null()
                            .default("pending"),
                    )
                    .col(
                        ColumnDef::new(StockItem::PriceHistory)
                            .json()
                            .not_null()
                            .default("[]"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(StockItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StockItem {
    Table,
    Id,
    WFMId,
    WFMUrl,
    ItemName,
    ItemUniqueName,
    SubType,
    Bought,
    MinimumPrice,
    ListPrice,
    Owned,
    IsHidden,
    CreatedAt,
    UpdatedAt,
    Status,
    PriceHistory,
}
