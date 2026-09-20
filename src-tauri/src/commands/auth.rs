use std::sync::{Arc, Mutex};

use qf_api::enums::ApplicationEvent;
use utils::{get_location, info, trace, warning, Error, LoggerOptions};

use crate::{
    app::{AppState, User},
    cache::client::CacheState,
    live_scraper::LiveScraperState,
    send_event, track_event,
    types::{PermissionsFlags, UIEvent},
    utils::{AuctionListExt, ErrorFromExt, OrderListExt},
};

// --------------------------------------------------
// Authentication
// --------------------------------------------------

#[tauri::command]
pub async fn auth_me(app: tauri::State<'_, Mutex<AppState>>) -> Result<User, Error> {
    let app = app.lock().unwrap();
    let mut user = app.user.clone();

    // Do not expose tokens
    user.qf_token.clear();
    user.wfm_token.clear();

    info(
        "Commands:AuthMe",
        &format!("Returning user {}", user.wfm_username),
        &LoggerOptions::default(),
    );

    Ok(user)
}

#[tauri::command]
pub async fn auth_login(
    email: String,
    password: String,
    app: tauri::State<'_, Mutex<AppState>>,
    cache: tauri::State<'_, Mutex<CacheState>>,
) -> Result<User, Error> {
    let app_state = app.lock()?.clone();
    let mut cache_state = cache.lock()?.clone();

    let result = async {
        let (qf_client, wfm_client, updated_user, ws, ws_chat) =
            app_state.login(&email, &password).await?;

        info(
            "Commands:AuthLogin",
            &format!("User {} logged in successfully", updated_user.wfm_username),
            &LoggerOptions::default(),
        );

        app_state.analytics.start();

        let (cache_version_id, price_version_id) = cache_state
            .load(&qf_client, app_state.settings.lang.clone())
            .await?;

        let mut app = app.lock()?;
        let mut cache = cache.lock()?;

        *cache = cache_state;
        cache.version.id = cache_version_id;
        cache.version.id_price = price_version_id;
        cache.version.save()?;

        wfm_client
            .order()
            .cache_orders_mut()
            .apply_item_info(&cache)?;

        wfm_client
            .auction()
            .cache_auctions_mut()
            .apply_item_info(&cache)?;

        app.wfm_client = wfm_client;
        app.user = updated_user.clone();
        app.set_qf_client(qf_client);
        app.wfm_socket = Some(ws);
        app.wfm_chat_socket = Some(ws_chat);

        send_event!(UIEvent::RefreshCache, "Cache refreshed successfully");

        Ok::<_, Error>(updated_user)
    }
    .await;

    match &result {
        Ok(_) => track_event!(
            ApplicationEvent::AuthLogin,
            [("success", "true".to_string())]
        ),
        Err(_err) => track_event!(
            ApplicationEvent::AuthLogin,
            [
                ("success", "false".to_string()),
                ("error_type", "login_failed".to_string()),
            ]
        ),
    }

    if let Err(e) = &result {
        e.log("auth_login.log");
    }

    result
}

#[tauri::command]
pub async fn auth_logout(
    app: tauri::State<'_, Mutex<AppState>>,
    live_scraper: tauri::State<'_, Arc<LiveScraperState>>,
) -> Result<User, Error> {
    let result = async {
        let app_state = app.lock()?.clone();

        live_scraper.stop();

        if let Some(ws) = &app_state.wfm_socket {
            ws.disconnect().map_err(|e| {
                Error::new(
                    "Commands:AuthLogout",
                    &format!("Failed to close WebSocket: {:?}", e),
                    get_location!(),
                )
            })?;
        }

        app_state.analytics.stop();

        let new_user = User::default();
        new_user.save()?;

        let mut app = app.lock()?;
        app.user = new_user.clone();

        Ok::<_, Error>(new_user)
    }
    .await;

    match &result {
        Ok(_) => track_event!(
            ApplicationEvent::AuthLogout,
            [("success", "true".to_string())]
        ),
        Err(_err) => track_event!(
            ApplicationEvent::AuthLogout,
            [
                ("success", "false".to_string()),
                ("error_type", "logout_failed".to_string()),
            ]
        ),
    }

    if let Err(e) = &result {
        e.log("auth_logout.log");
    }

    result
}

#[tauri::command]
pub async fn auth_has_permission(
    flag: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<bool, Error> {
    let app_state = app.lock().unwrap().clone();

    if let Err(_) = app_state
        .user
        .has_permission(PermissionsFlags::from_str(&flag))
    {
        warning(
            "Commands:AuthHasPermission",
            &format!("User does not have permission for flag: {}", flag),
            &LoggerOptions::default(),
        );

        return Ok(false);
    }
    Ok(true)
}
