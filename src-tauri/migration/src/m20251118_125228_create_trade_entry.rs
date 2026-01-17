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
                    .table(TradeEntry::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TradeEntry::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TradeEntry::WFMId).uuid().not_null())
                    .col(ColumnDef::new(TradeEntry::Name).string().not_null())
                    .col(ColumnDef::new(TradeEntry::SubType).json())
                    .col(
                        ColumnDef::new(TradeEntry::Price)
                            .integer()
                            .not_null()
                            .default(Value::Int(Some(0))),
                    )
                    .col(ColumnDef::new(TradeEntry::Tags).string().not_null())
                    .col(ColumnDef::new(TradeEntry::Group).string().not_null())
                    .col(ColumnDef::new(TradeEntry::Properties).json())
                    .col(ColumnDef::new(TradeEntry::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(TradeEntry::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(TradeEntry::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TradeEntry {
    Table,
    Id,
    WFMId,
    Name,
    SubType,
    Price,
    Tags,
    Group,
    Properties,
    CreatedAt,
    UpdatedAt,
}
