use std::sync::{Arc, Mutex};

use actix_web::{middleware::Logger, post, web, HttpResponse, Responder};
use migration::index;
use regex::Regex;
use tauri::{Manager, State};

use crate::{
    cache::{client::CacheClient, types::cache_item_component::CacheItemComponent},
    http_client::types::trade::{PlayerTrade, TradeItem},
    notification::client::NotifyClient,
    utils::modules::logger,
    APP,
};

#[post("/progress")]
pub async fn progress(mut riven: web::Json<PlayerTrade>) -> impl Responder {
    let app_handle = APP.get().expect("failed to get app handle");
    // let app_state: State<Arc<Mutex<AppState>>> = app_handle.state();
    // let app = app_state.lock().expect("failed to lock app state");
    let notify_state: State<Arc<Mutex<NotifyClient>>> = app_handle.state();
    let _notify = notify_state.lock().expect("failed to lock notify state");
    let cache_state: State<Arc<Mutex<CacheClient>>> = app_handle.state();
    let cache = cache_state.lock().expect("failed to lock notify state");

    for item in riven.offered_items.iter_mut() {
        if !parse_item(&cache, item) {            
            logger::warning_file("TradeProgress", &format!("Failed to parse item: {}", item.name), Some("trade_progress.log"));
        }else {
            // Get trade item from cache
            let trade_item = cache.tradable_items().find_item(item.unique_name.as_str(), "--item_by unique_name --item_lang en").expect("Failed to find item");
            if trade_item.is_some() && item.name != "Platinum" {
                item.wfm_id = Some(trade_item.as_ref().unwrap().wfm_id.clone());
                item.wfm_url = Some(trade_item.as_ref().unwrap().wfm_url_name.clone());
            }
        }
    }
    for item in riven.received_items.iter_mut() {
        if !parse_item(&cache, item) {
            logger::warning_file("TradeProgress", &format!("Failed to parse item: {}", item.name), Some("trade_progress.log"));
        } else {
            // Get trade item from cache
            let trade_item = cache.tradable_items().find_item(item.unique_name.as_str(), "--item_by unique_name --item_lang en").expect("Failed to find item");
            if trade_item.is_some() && item.name != "Platinum" {
                item.wfm_id = Some(trade_item.as_ref().unwrap().wfm_id.clone());
                item.wfm_url = Some(trade_item.as_ref().unwrap().wfm_url_name.clone());
            }
        }
    }
    match riven.trade_type.as_str() {
        "Trade" => {}
        "Sale" => {
            items_were_just_traded(cache.to_owned(), riven.offered_items.clone(), &riven.player_name, true, riven.platinum.unwrap_or(0));
        }
        "Purchase" => {
            items_were_just_traded(cache.to_owned(), riven.received_items.clone(), &riven.player_name, false, riven.platinum.unwrap_or(0));
        }
        _ => {            
            logger::warning_file("TradeProgress", &format!("Failed to parse trade type: {}", riven.trade_type), Some("trade_progress.log"));
        }
        
    }
    HttpResponse::Ok().body(format!("Hello {}!", riven.player_name))
}

fn parse_item(cache: &CacheClient, item: &mut TradeItem) -> bool {
    if item.name == "plat" {
        item.name = "Platinum".to_string();
        item.unique_name = "/QF_Special/Platinum".to_string();
        return true;
    }
    if item.name.starts_with("Imprint of") {
        item.unique_name = format!(
            "/QF_Special/Imprint/{}",
            item.name.replace("Imprint of ", "")
        );        
        return true;
    }
    if item.name.starts_with("Legendary Core") {
        item.unique_name = "/QF_Special/Legendary Fusion Core".to_string();
        return true;
    }

    if item.name.starts_with("Ancient Core") {
        item.name = "/QF_Special/Legendary Ancient Core".to_string();
        return true;
    }

    // Check Prime Parts
    let prime_part = cache.relics().get_relic_by_name(&item.name.to_lowercase());
    if let Some(prime_part) = prime_part {
        item.name = prime_part.name.clone();
        item.unique_name = prime_part.unique_name.clone();
        return true;
    }

    // Check Misc items
    let misc_item = cache.misc().get_by_name(&item.name, false);
    if let Some(misc_item) = misc_item {
        item.name = misc_item.name.clone();
        item.unique_name = misc_item.unique_name;
        return true;
    }
    // Check Mods, Rivens, Fish
    if item.name.contains("(") && item.name.contains(")") {
        let index = item.name.find("(").unwrap() as usize;
        let rank_part = &item.clone().name[index..];
        let name_part = &item.clone().name[..index - 1];

        // Check if the item is a mod/fish true if mod else it is a fish
        if rank_part.len() > 3 {
            let rank_part = rank_part.replace("(", "").replace(")", "");
            // Get The Rank of the mod
            for s in rank_part.split(' ') {
                if let Ok(result) = s.parse::<i64>() {
                    item.rank = Some(result);
                    break;
                }
            }
            // Check if the item is a riven
            if item.name.contains("(RIVEN RANK ") {
                // Check if the item is a veiled riven
                if item.name.contains(" Riven Mod") {
                    let all_mods = cache.mods().get_items();
                    let mut filters = all_mods
                        .iter()
                        .filter(|x| x.name.contains(name_part))
                        .collect::<Vec<_>>();
                    filters.sort_by_key(|key| !key.unique_name.contains("/Raw"));

                    let cache_mod = filters.first();
                    if let Some(cache_mod) = cache_mod {
                        item.name = cache_mod.name.clone();
                        item.unique_name = cache_mod.unique_name.clone();
                    } else {
                        return false;
                    }
                } else {
                    let last_space_index = name_part.find(" ").unwrap() as usize;
                    let weapon = &name_part[..last_space_index];
                    let att = &name_part[last_space_index + 1..];
                    item.name = format!("{} Riven Mod ({})", weapon, att);
                    item.unique_name = format!("/QF_Special/Riven/{}/{}", weapon, att);
                    return true;
                }
            } else {
                let cache_mod = cache.mods().get_by_name(&name_part, true);
                if let Some(cache_mod) = cache_mod {
                    item.name = cache_mod.name.clone();
                    item.unique_name = cache_mod.unique_name.clone();
                    return true;
                }
            }
        } else {
            let cache_fish = cache.fish().get_by_name(&name_part, true);
            let size = rank_part.replace("(", "").replace(")", "");
            if size.len() == 1 {
                if let Some(c) = size.chars().next() {
                    item.rank = Some(c as i64);
                }
            }
            if let Some(cache_fish) = cache_fish {
                item.name = cache_fish.name.clone();
                item.unique_name = cache_fish.unique_name.clone();
                return true;
            }
        }
    }
    // Check Arcane
    if item.name.len() != item.name.chars().count() || item.name.ends_with("???") {
        let index = item.name.rfind(' ').unwrap_or(0);
        let name_part = &item.name[..index];
        let cache_arcane = cache.arcane().get_by_name(&name_part, true);
        if let Some(cache_arcane) = cache_arcane {
            item.name = cache_arcane.name.clone();
            item.unique_name = cache_arcane.unique_name.clone();
            return true;
        }else {
            item.name = name_part.to_string();
        }
    }
    if item.name == "Enter Nihil's Oubliette".to_string() {
        item.unique_name = "/QF_Special/Other/Nihil's Oubliette (Key)".to_string();
        return true;
    }
    // Warframe Parts
    let warframe_part = cache.warframe().get_part(&item.name);
    if let Some(warframe_part) = warframe_part {
        item.name = warframe_part.name.clone();
        item.unique_name = warframe_part.unique_name.clone();
        return true;
    }
    // Weapon Parts
    let weapon_part =
        cache
            .parts()
            .get_part_by_name("All", &item.name.replace(" Blueprint", ""), true);
    if let Some(weapon_part) = weapon_part {
        item.name = weapon_part.name.clone();
        item.unique_name = weapon_part.unique_name.clone();
        return true;
    }

    // Melee Weapons
    let melee = cache.melee().get_by_name(&item.name, true);
    if let Some(melee) = melee {
        item.name = melee.name.clone();
        item.unique_name = melee.unique_name.clone();
        return true;
    }

    // Primary Weapons
    let primary = cache.primary().get_by_name(&item.name, true);
    if let Some(primary) = primary {
        item.name = primary.name.clone();
        item.unique_name = primary.unique_name.clone();
        return true;
    }

    // Secondary Weapons
    let secondary = cache.secondary().get_by_name(&item.name, true);
    if let Some(secondary) = secondary {
        item.name = secondary.name.clone();
        item.unique_name = secondary.unique_name.clone();
        return true;
    }

    // Archwing
    let archwing = cache.archwing().get_by_name(&item.name, true);
    if let Some(archwing) = archwing {
        item.name = archwing.name.clone();
        item.unique_name = archwing.unique_name.clone();
        return true;
    }

    // Archwing Guns
    let arch_gun = cache.arch_gun().get_by_name(&item.name, true);
    if let Some(arch_gun) = arch_gun {
        item.name = arch_gun.name.clone();
        item.unique_name = arch_gun.unique_name.clone();
        return true;
    }

    // Archwing Melee
    let arch_melee = cache.arch_melee().get_by_name(&item.name, true);
    if let Some(arch_melee) = arch_melee {
        item.name = arch_melee.name.clone();
        item.unique_name = arch_melee.unique_name.clone();
        return true;
    }

    // Relics
    if item.name.contains("Relic") {
        let mut str = item.name.replace(" Relic", "");
        if str.split(' ').count() == 2 {
            str += " [INTACT]";
        }
        let compare_name = str.replace("[", "").replace("]", "").to_lowercase();
        let relic = cache.relics().get_by_name(&compare_name, true);
        if let Some(relic) = relic {
            item.name = relic.name.clone();
            item.unique_name = relic.unique_name.clone();
            return true;
        }
    }
    // Skins
    let skin = cache.skin().get_by_name(&item.name, true);
    if let Some(skin) = skin {
        item.name = skin.name.clone();
        item.unique_name = skin.unique_name.clone();
        return true;
    }

    // Misc Items
    let m_items = cache.misc().get_items();
    let misc_items = m_items.iter().filter(|x| x.name.contains(&item.name)).collect::<Vec<_>>();
    if let Some(mi_item) = misc_items.first() {
        let components = mi_item.components.as_ref();
        if let Some(components) = components {
            for component in components {
                let com_name = format!("{} {}", mi_item.name, component.name);
                if com_name == item.name.replace(" Blueprint", "") {
                    item.unique_name = component.unique_name.clone();
                    break;
                }
            }
        }
    }
    // Misc Items
    let misc_item =cache.misc().get_by_name(&item.name.replace(" Blueprint", ""), true);
    if let Some(misc_item) = misc_item {
        item.name = misc_item.name.clone();
        item.unique_name = misc_item.unique_name.clone();
        return true;
    }
    // Pets 
    let pet = cache.pet().get_by_name(&item.name.replace(" Blueprint", ""), true);
    if let Some(pet) = pet {
        item.name = pet.name.clone();
        item.unique_name = pet.unique_name.clone();
        return true;
    }
    // Resources
    let resource = cache.resource().get_by_name(&item.name, true);
    if let Some(resource) = resource {
        item.name = resource.name.clone();
        item.unique_name = resource.unique_name.clone();
        return true;
    }
    item.unique_name = format!("/QF_Special/Other/{}", item.name);
    false
}

fn items_were_just_traded(cache: CacheClient, items: Vec<TradeItem>, user_name: &str, selling: bool, platinum: i64){    
    let all_items_parts = cache.parts().get_parts("Weapon");

    let mut final_item_name = "".to_string();
    let mut final_item_internal_name = "".to_string();
    let mut quantity = 0;

    if items.len() > 1 {
        let mut source: Vec<(Option<&CacheItemComponent>, i64)> = vec![];
        for item in items {
             let com = all_items_parts.iter().find(|x| x.unique_name == item.unique_name);
             source.push((com.clone(), item.quantity));
        }
        // Get first item in the list
        let first_item = source.first();
        
        // Check first item is not none
        if first_item.is_none() || first_item.unwrap().0.is_none() || first_item.unwrap().0.unwrap().part_of.is_none() {
            
            logger::warning_con("TradeProgress", "Sold items conversion to set failed due to missing part_of");
            return;
        }
        let main_part = first_item.unwrap().0.unwrap().part_of.as_ref().unwrap();

        // if first_item.is_some() && source.iter().all(|x| x.0.is_some() && x.0.unwrap().part_of.is_some() && x.0.unwrap().part_of.as_ref().unwrap().unique_name == first_item.unwrap().0.unwrap().unique_name) {
         if first_item.is_some() && source.iter().all(|x| 
            {
                let com = x.0;

                if com.is_none() || com.unwrap().part_of.is_none() {
                    return false;                    
                }
                let com = com.unwrap();
                let part_of = com.part_of.as_ref().unwrap();
                return part_of.unique_name == main_part.unique_name;
            }
        ) {
            let mut num = source.iter().map(|(_, count)| count).min().cloned().unwrap_or(0);
            if main_part.name.to_lowercase().contains("dual decurion"){
                num /= 2
            }
            final_item_name = format!("{} Set", main_part.name);
            final_item_internal_name = main_part.unique_name.clone();
            quantity = num;
            logger::info_con("TradeProgress", &format!("Sold items conversion to set successful for {}", final_item_name));
        }
        else {
            logger::warning_con("TradeProgress", "Sold items conversion to set failed due to different parts");
        }
    } else {
        let item = cache.tradable_items().find_item(items.first().unwrap().unique_name.as_str(),"--item_by unique_name --item_lang en").expect("Failed to find item");
        if item.is_none() {
            logger::warning_con("TradeProgress", "Failed to find item");
            return;
        }
        let item = item.unwrap();
        final_item_name = item.name.clone();
        final_item_internal_name = item.unique_name.clone();
        quantity = items.first().unwrap().quantity;
    }
}