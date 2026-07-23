use std::sync::{Arc, Mutex};

use serde_json::{json, Value};
use utils::{Error, Properties, ZipOptions};

use crate::{
    app::{AppState, user},
    helper,
    live_scraper::LiveScraperState,
    log_parser::LogParserState,
    utils::modules::states::get_app_error,
    APP, SENSITIVE_FIELDS,
};

#[tauri::command]
pub async fn log_export(
    app: tauri::State<'_, Mutex<AppState>>,
    log_parser: tauri::State<'_, Mutex<Arc<LogParserState>>>,
    live_scraper: tauri::State<'_, Arc<LiveScraperState>>,
) -> Result<String, Error> {
    let app = app.lock()?.clone();
    let log_parser = log_parser.lock()?;
    let app_info = APP.get().expect("App handle not found");
    let info = app_info.package_info().clone();
    let date = chrono::Local::now()
        .naive_utc()
        .format("%Y-%m-%d")
        .to_string();

    let version = info.version.to_string();
    let app_path = helper::get_app_storage_path();

    utils::export_cached_logs(&app_path)?;

    let zip_path =
        helper::get_desktop_path().join(format!("{} v{} {} Logs.zip", info.name, version, date));

    let mut zip = ZipOptions::new();
    zip.exclude_patterns(&["EBWebView/"]);
    zip.mask_properties(SENSITIVE_FIELDS);
    zip.create_file(
        "AppError.json",
        json!(get_app_error()).to_string().as_bytes(),
    );
    zip.create_file(
        "LogParser.json",
        json!({
            "totalLines": log_parser.get_all_cached_lines().len(),
        })
        .to_string()
        .as_bytes(),
    );
    zip.create_file(
        "LiveScraper.json",
        json!({
            "isRunning": live_scraper.is_running(),
            "justStarted": live_scraper.just_started(),
        })
        .to_string()
        .as_bytes(),
    );
    zip.create_file(
        "AppInfo.json",
        json!({
        "isDevelopment": app.is_development,
        "isPreRelease": app.is_pre_release,
        })
        .to_string()
        .as_bytes(),
    );
    let mut wfm_info = Properties::default();
    wfm_info.set_property_value("tracking", app.wfm_client.get_tracking());
    let wfm_user = app.wfm_client.get_user().ok();
    if wfm_user.is_some() {
        let user = wfm_user.unwrap();
        wfm_info.set_property_value("wfmTier", format!("{:?}", user.tier));
        wfm_info.set_property_value("wfmSubscription", user.subscription);
    }
    wfm_info.set_property_value(
        "wfmTotalOrders",
        app.wfm_client.order().cache_orders().total_orders(),
    );
    wfm_info.set_property_value(
        "wfmTotalBuyOrders",
        app.wfm_client.order().cache_orders().buy_orders.len(),
    );
    wfm_info.set_property_value(
        "wfmTotalSellOrders",
        app.wfm_client.order().cache_orders().sell_orders.len(),
    );
    let per_rate_limit = app.wfm_client.get_per_route_limiter().clone();
    for (key, route) in per_rate_limit.lock()?.iter() {
        wfm_info.update_property("limiters", |data: &mut Vec<String>| {
            data.push(format!(
                "{}: Limit: {} | Wait Time (sec): {}, Quota Type: {}",
                key,
                route.quota_type.current_limit(),
                route.wait_time_sec,
                route.quota_type.quota_type()
            ));
        });
    }
    zip.create_file(
        "WfmInfo.json",
        json!(wfm_info.properties).to_string().as_bytes(),
    );
    Ok(zip.create_zip(app_path, zip_path)?)
}

#[tauri::command]
pub async fn log(
    cause: String,
    component: String,
    location: String,
    log_level: String,
    message: String,
    context: Option<Value>,
) -> Result<(), Error> {
    let error = Error::new(component, message, location)
        .with_context(context.unwrap_or_default())
        .with_cause(cause)
        .set_log_level(utils::LogLevel::from_str(&log_level));

    error.log("log.log");
    Ok(())
}
