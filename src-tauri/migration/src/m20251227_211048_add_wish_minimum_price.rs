use crate::m20240406_135323_create_wish_list_table::WishList;
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(WishList::Table)
                    .add_column(ColumnDef::new(WishList::MinimumPrice).integer())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(WishList::Table)
                    .drop_column(WishList::MinimumPrice)
                    .to_owned(),
            )
            .await
    }
}
