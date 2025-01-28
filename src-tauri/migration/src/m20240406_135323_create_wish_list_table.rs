use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WishList::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WishList::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WishList::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(WishList::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(WishList::WFMId).uuid().not_null())
                    .col(ColumnDef::new(WishList::WFMUrl).string().not_null())
                    .col(ColumnDef::new(WishList::ItemName).string().not_null())
                    .col(
                        ColumnDef::new(WishList::ItemUniqueName)
                            .string()
                            .not_null()
                            .default("N/A"),
                    )
                    .col(ColumnDef::new(WishList::SubType).json())
                    .col(
                        ColumnDef::new(WishList::Quantity)
                            .integer()
                            .not_null()
                            .default(Value::Int(Some(1))),
                    )
                    .col(ColumnDef::new(WishList::MaximumPrice).integer())
                    .col(ColumnDef::new(WishList::ListPrice).integer())
                    .col(
                        ColumnDef::new(WishList::PriceHistory)
                            .json()
                            .not_null()
                            .default("[]"),
                    )
                    .col(
                        ColumnDef::new(WishList::Status)
                            .string()
                            .not_null()
                            .default("pending"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WishList::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum WishList {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    WFMId,
    WFMUrl,
    ItemName,
    ItemUniqueName,
    SubType,
    Quantity,
    MaximumPrice,
    ListPrice,
    PriceHistory,
    Status,
    IsHidden,
}
