use std::sync::{atomic::Ordering, Arc, Mutex};

use serde_json::{json, Value};
use utils::Error;

use crate::{
    add_metric,
    app::StockItemSettings,
    cache::{CacheState, ItemPriceInfo},
    live_scraper::{self, LiveScraperState},
    send_event,
    types::*,
};

#[tauri::command]
pub async fn live_scraper_toggle(
    live_scraper: tauri::State<'_, Arc<LiveScraperState>>,
) -> Result<(), Error> {
    if live_scraper.is_running.load(Ordering::SeqCst) {
        live_scraper.stop();
        add_metric!("live_scraper_toggle", "stopped");
    } else {
        live_scraper.start();
        add_metric!("live_scraper_toggle", "started");
    }
    send_event!(
        UIEvent::UpdateLiveScraperRunningState,
        json!(live_scraper.is_running.load(Ordering::SeqCst))
    );
    Ok(())
}
#[tauri::command]
pub async fn live_scraper_get_state(
    live_scraper: tauri::State<'_, Arc<LiveScraperState>>,
) -> Result<Value, Error> {
    Ok(json!({
        "is_running": live_scraper.is_running.load(Ordering::SeqCst)
    }))
}
#[tauri::command]
pub async fn live_scraper_get_interesting_wtb_items(
    settings: StockItemSettings,
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<Vec<ItemPriceInfo>, Error> {
    let mut items = live_scraper::helpers::get_interesting_items(&settings);
    let cache = cache.lock()?;
    for item in &mut items {
        if let Ok(info) = cache.tradable_item().get_by(&item.wfm_id) {
            item.properties.set_property_value("name", info.name);
        }
    }
    Ok(items)
}
