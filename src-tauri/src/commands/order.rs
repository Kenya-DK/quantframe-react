use std::sync::{Arc, Mutex};

use utils::{get_location, Error};

use crate::{
    app::client::AppState,
    live_scraper::LiveScraperState,
    utils::{ErrorFromExt, OrderListExt},
};

#[tauri::command]
pub async fn order_refresh(app: tauri::State<'_, Mutex<AppState>>) -> Result<(), Error> {
    let app = app.lock()?.clone();
    app.wfm_client.order().my_orders().await.map_err(|e| {
        let err = Error::from_wfm(
            "OrderRefresh",
            "Failed to refresh orders",
            e,
            get_location!(),
        );
        err.log(Some("order_refresh.log"));
        err
    })?;
    app.wfm_client
        .order()
        .cache_orders_mut()
        .apply_trade_info()?;
    Ok(())
}

#[tauri::command]
pub async fn order_delete_all(
    live_scraper: tauri::State<'_, Arc<LiveScraperState>>,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let app = app.lock()?.clone();
    live_scraper.stop();
    let orders = match app.wfm_client.order().my_orders().await {
        Ok(orders) => orders,
        Err(e) => {
            let err = Error::from_wfm("OrderDeleteAll", "Failed to get orders", e, get_location!());
            err.log(Some("order_delete_all.log"));
            return Err(err);
        }
    };
    for order in orders.to_vec() {
        if let Err(e) = app.wfm_client.order().delete(&order.id).await {
            let err = Error::from_wfm(
                "OrderDeleteAll",
                "Failed to delete order",
                e,
                get_location!(),
            );
            err.log(Some("order_delete_all.log"));
            return Err(err);
        }
    }
    Ok(())
}
