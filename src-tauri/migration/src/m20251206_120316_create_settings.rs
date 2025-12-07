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
                    .table(Setting::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Setting::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Setting::Key).string().not_null())
                    // Maximum length of 65535 characters for TEXT type
                    .col(ColumnDef::new(Setting::Value).text().not_null())
                    .col(ColumnDef::new(Setting::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Setting::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Setting::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Setting {
    Table,
    Id,
    Key,
    Value,
    CreatedAt,
    UpdatedAt,
}
