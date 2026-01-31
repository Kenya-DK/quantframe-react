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
                    .table(Transaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transaction::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Transaction::WFMId).uuid().not_null())
                    .col(ColumnDef::new(Transaction::WFMUrl).string().not_null())
                    .col(ColumnDef::new(Transaction::ItemName).string().not_null())
                    .col(ColumnDef::new(Transaction::ItemType).string().not_null())
                    .col(
                        ColumnDef::new(Transaction::ItemUniqueName)
                            .string()
                            .not_null()
                            .default("N/A"),
                    )
                    .col(ColumnDef::new(Transaction::SubType).json())
                    .col(ColumnDef::new(Transaction::Tags).string().not_null())
                    .col(
                        ColumnDef::new(Transaction::TransactionType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Transaction::Quantity)
                            .integer()
                            .not_null()
                            .default(Value::Int(Some(1))),
                    )
                    .col(
                        ColumnDef::new(Transaction::UserName)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(Transaction::Price)
                            .integer()
                            .not_null()
                            .default(Value::Int(Some(0))),
                    )
                    .col(ColumnDef::new(Transaction::Properties).json())
                    .col(
                        ColumnDef::new(Transaction::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Transaction::UpdatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transaction::CreatedAt).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Transaction {
    Table,
    Id,
    WFMId,
    WFMUrl,
    ItemName,
    ItemType,
    ItemUniqueName,
    SubType,
    Tags,
    TransactionType,
    Price,
    Quantity,
    UserName,
    Profit,
    Credits,
    CreatedAt,
    UpdatedAt,
    Properties,
}
