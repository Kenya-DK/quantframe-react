use ::entity::transaction::transaction_wat;
use ::entity::transaction::{transaction, transaction::Entity as Transaction};
use ::entity::transaction::{
    transaction_old, transaction_old::Entity as TransactionOld,
    transaction_wat::Entity as TransactionWat,
};

use sea_orm::*;

pub struct TransactionQuery;

impl TransactionQuery {
    pub async fn get_all(db: &DbConn) -> Result<Vec<transaction::Model>, DbErr> {
        Transaction::find().all(db).await
    }

    pub async fn get_wat_transactions(db: &DbConn) -> Result<Vec<transaction_wat::Model>, DbErr> {
        TransactionWat::find().all(db).await
    }
    pub async fn get_old_transactions(db: &DbConn) -> Result<Vec<transaction_old::Model>, DbErr> {
        TransactionOld::find().all(db).await
    }

    pub async fn find_by_id(db: &DbConn, id: i64) -> Result<Option<transaction::Model>, DbErr> {
        Transaction::find_by_id(id).one(db).await
    }
}
