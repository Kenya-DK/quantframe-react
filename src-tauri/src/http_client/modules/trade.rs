use std::sync::{Arc, Mutex};

use actix_web::{post, web, HttpResponse, Responder};
use tauri::{Manager, State};
use regex::Regex;

use  entity::stock::riven::create::CreateStockRiven;

use crate::{
    http_client::
    app::client::AppState, cache::{client::CacheClient, types::cache_riven::RivenStat}, notification::client::NotifyClient, APP
};

#[post("/progress")]
pub async fn progress(mut riven: web::Json<PlayerTrade>) -> impl Responder {
    let re = Regex::new(r"<.*?>").unwrap();
    let app_handle = APP.get().expect("failed to get app handle");
    let app_state: State<Arc<Mutex<AppState>>> = app_handle.state();
    let app = app_state.lock().expect("failed to lock app state");
    let notify_state: State<Arc<Mutex<NotifyClient>>> = app_handle.state();
    let notify = notify_state.lock().expect("failed to lock notify state");
    let cache_state: State<Arc<Mutex<CacheClient>>> = app_handle.state();
    let cache = cache_state.lock().expect("failed to lock notify state");

    // Find the weapon in the cache
    let weapon = cache
        .riven()
        .get_wfm_riven_type_by_name(&riven.wfm_url, "en");
    if weapon.is_none() {
        return HttpResponse::Ok().body(format!("Weapon not found: {}", riven.wfm_url));
    }
    riven.wfm_url = weapon.clone().unwrap().wfm_url_name.clone();
    
    // Find the attributes in the cache
    let upgrades: Vec<RivenStat> = cache.riven().get_weapon_upgrades(&weapon.unwrap().unique_name).unwrap().values().cloned().collect();
    
    for att in riven.attributes.iter_mut()  {
        // Remove everything between < and >
        let upgrade = upgrades.iter().find(|x| re.replace_all(&x.short_string, "").to_string() == att.url_name);
        if upgrade.is_none() {
            return HttpResponse::Ok().body(format!("Attribute not found: {}", att.url_name));
        }
        att.url_name= upgrade.unwrap().wfm_id.clone();
    }

    println!("Weapon: {:?}", riven);

    // let json = json!(riven.clone());
    // notify.gui().send_event(
    //     crate::utils::enums::ui_events::UIEvent::UpdateStockRivens,
    //     Some(json),
    // );
    // println!("Weapon Type: {:?}", riven);
    HttpResponse::Ok().body(format!("Hello {}!", riven.weapon_type))
}

#[post("/add_item")]
pub async fn add_item() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}
