use std::sync::{atomic::Ordering, Arc, Mutex};

use qf_api::types::*;
use utils::{get_location, Error};

use crate::{
    app::client::AppState, live_scraper::LiveScraperState, utils::modules::states::ErrorFromExt,
};

#[tauri::command]
pub async fn live_scraper_toggle(
    live_scraper: tauri::State<'_, Arc<LiveScraperState>>,
) -> Result<(), Error> {
    println!("Toggling live scraper");

    if live_scraper.is_running.load(Ordering::SeqCst) {
        println!("Stopping live scraper");
        live_scraper.stop();
    } else {
        println!("Starting live scraper");
        live_scraper.start();
    }

    Ok(())
}
