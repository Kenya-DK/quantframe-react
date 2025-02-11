use std::sync::{Arc, Mutex};

use entity::sub_type::SubType;
use serde_json::Value;

use crate::{
    qf_client::{
        client::QFClient,
        types::{
            item_price::ItemPrice, item_price_chat::ItemPriceChat, paginated::Paginated,
            paginated_with_include::PaginatedWithInclude, syndicates_price::SyndicatesPrice,
        },
    },
    utils::modules::error::AppError,
};

#[tauri::command]
pub async fn item_get_syndicates_prices(
    page: i64,
    limit: i64,
    sort: Option<Value>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<Paginated<SyndicatesPrice>, AppError> {
    let qf = qf.lock().expect("Could not lock qf").clone();
    match qf.item().get_syndicates_prices(page, limit, sort).await {
        Ok(list) => Ok(list),
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn item_get_prices(
    page: i64,
    limit: i64,
    from_date: String,
    to_date: String,
    order_type: Option<String>,
    wfm_url: Option<String>,
    sub_type: Option<SubType>,
    include: Option<String>,
    group_by: Option<String>,
    sort: Option<Value>,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<PaginatedWithInclude<ItemPrice, ItemPriceChat>, AppError> {
    let qf = qf.lock().expect("Could not lock qf").clone();
    match qf
        .item()
        .get_prices(
            page, limit, from_date, to_date, order_type, wfm_url, sub_type, include, group_by, sort,
        )
        .await
    {
        Ok(list) => Ok(list),
        Err(e) => Err(e),
    }
}
