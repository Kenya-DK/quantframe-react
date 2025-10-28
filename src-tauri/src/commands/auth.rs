use std::sync::{Arc, Mutex};

use utils::{get_location, info, warning, Error, LoggerOptions};

use crate::{
    app::{client::AppState, User},
    cache::client::CacheState,
    live_scraper::LiveScraperState,
    types::PermissionsFlags,
    utils::{AuctionListExt, ErrorFromExt, OrderListExt},
};

#[tauri::command]
pub async fn auth_me(app: tauri::State<'_, Mutex<AppState>>) -> Result<User, Error> {
    let app = app.lock().unwrap();
    let mut user = app.user.clone();
    user.qf_token = String::new(); // Do not expose the token
    user.wfm_token = String::new(); // Do not expose the token
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

    let (qf_client, wfm_client, updated_user, ws, ws_chat) =
        match app_state.login(&email, &password).await {
            Ok((qf_client, wfm_client, user, ws, ws_chat)) => {
                (qf_client, wfm_client, user, ws, ws_chat)
            }
            Err(e) => {
                e.log("auth_login.log");
                return Err(e);
            }
        };
    // Update The current AuthState
    info(
        "Commands:AuthLogin",
        &format!("User {} logged in successfully", updated_user.wfm_username),
        &LoggerOptions::default(),
    );
    qf_client
        .analytics()
        .add_metric("login", updated_user.wfm_username.as_str());
    qf_client.analytics().start().map_err(|e| {
        Error::from_qf(
            "AppState:Validate",
            "Failed to start QF analytics",
            e,
            get_location!(),
        )
    })?;

    let (cache_version_id, price_version_id) = match cache_state.load(&qf_client).await {
        Ok((cache_version_id, price_version_id)) => (cache_version_id, price_version_id),
        Err(e) => {
            e.log("auth_login.log");
            return Err(e);
        }
    };
    let mut app = app.lock()?;
    let mut cache = cache.lock()?;
    *cache = cache_state;
    cache.version.id = cache_version_id;
    cache.version.id_price = price_version_id;
    cache.version.save()?;
    // Apply item info to WFM client
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
    app.qf_client = qf_client;
    app.wfm_socket = Some(ws);
    app.wfm_chat_socket = Some(ws_chat);

    Ok(updated_user)
}

#[tauri::command]
pub async fn auth_logout(
    app: tauri::State<'_, Mutex<AppState>>,
    live_scraper: tauri::State<'_, Arc<LiveScraperState>>,
) -> Result<User, Error> {
    let app_state = app.lock().unwrap().clone();
    live_scraper.stop();
    // Stop the WebSocket if it exists
    if let Some(ws) = &app_state.wfm_socket {
        match ws.disconnect() {
            Ok(_) => info(
                "Commands:AuthLogout",
                "WebSocket disconnected successfully",
                &LoggerOptions::default().set_file("auth_logout.log"),
            ),
            Err(e) => {
                let err = Error::new(
                    "Commands:AuthLogout",
                    &format!("Failed to close WebSocket: {:?}", e),
                    get_location!(),
                );
                err.log("auth_logout.log");
                return Err(err);
            }
        }
    }

    app_state
        .qf_client
        .analytics()
        .add_metric("logout", app_state.user.wfm_username.clone());
    match app_state.qf_client.analytics().send_current_metrics().await {
        Ok(_) => info(
            "Commands:AuthLogout",
            "Successfully sent current metrics",
            &LoggerOptions::default(),
        ),
        Err(e) => {
            let err = Error::from_qf(
                "Commands:AuthLogout",
                "Failed to send current metrics",
                e,
                get_location!(),
            );
            err.log("auth_logout.log");
            return Err(err);
        }
    }
    // Stop QF analytics if it exists
    app_state.qf_client.analytics().stop();

    let new_user = User::default();
    new_user
        .save()
        .map_err(|e| {
            e.log("auth_logout.log");
        })
        .unwrap();
    // Update The current AuthState
    let mut app = app.lock().expect("Could not lock auth");
    app.user = new_user.clone();
    Ok(new_user)
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
    info(
        "Commands:AuthHasPermission",
        &format!("User has permission for flag: {}", flag),
        &LoggerOptions::default(),
    );
    return Ok(true);
}
