use chrono::{DateTime, Utc};
use entity::{
    dto::{FinancialGraph, FinancialReport, PaginatedResult, SubType},
    enums::RivenGrade,
    stock_riven::RivenAttribute,
    transaction::TransactionPaginationQueryDto,
};
use serde_json::{json, Value};
use service::TransactionQuery;
use std::{
    collections::HashMap,
    fs::{self},
    path::PathBuf,
};
use tauri::{Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use utils::*;
use wf_market::{
    enums::OrderType,
    types::{AuctionLike, Order},
    Authenticated,
};

use crate::{
    cache::{
        apply_rank_multiplier, compute_riven_endo_cost, compute_riven_kuva_cost,
        count_riven_positive_and_negative_stats, derive_riven_summary_attributes, grade_riven,
        lookup_riven_multipliers, scale_attributes, CacheRivenWeapon, CacheState,
        CacheTradableItem,
    },
    types::OperationSet,
    utils::{
        auction_list_ext::AuctionWithOwnerListExt, modules::states, ErrorFromExt, OrderExt,
        OrderListExt, SubTypeExt,
    },
    APP, DATABASE,
};

pub static APP_PATH: &str = "dev.kenya.quantframe";

pub fn get_device_id() -> String {
    let app = APP.get().unwrap();
    let home_dir = match app.path().home_dir() {
        Ok(val) => val,
        Err(_) => {
            panic!("Could not find home directory");
        }
    };
    let device_name = home_dir.file_name().unwrap().to_str().unwrap();
    device_name.to_string()
}
pub fn get_app_storage_path() -> PathBuf {
    let app = APP.get().unwrap();
    let local_path = match app.path().local_data_dir() {
        Ok(val) => val,
        Err(_) => {
            panic!("Could not find app path");
        }
    };

    let app_path = local_path.join(APP_PATH);
    if !app_path.exists() {
        fs::create_dir_all(&app_path).unwrap()
    }
    app_path
}

pub fn get_sounds_path() -> PathBuf {
    let sounds_path = get_app_storage_path().join("sounds");
    if !sounds_path.exists() {
        fs::create_dir_all(&sounds_path).unwrap()
    }
    sounds_path
}

pub fn get_desktop_path() -> PathBuf {
    let app = APP.get().unwrap();
    let desktop_path = match app.path().desktop_dir() {
        Ok(val) => val,
        Err(_) => {
            panic!("Could not find desktop path");
        }
    };
    desktop_path
}
pub fn generate_transaction_summary(
    transactions: &Vec<entity::transaction::Model>,
    date: DateTime<Utc>,
    group_by1: GroupByDate,
    group_by2: &[GroupByDate],
    _previous: bool,
) -> (FinancialReport, FinancialGraph<i64>) {
    let (start, end) = get_start_end_of(date, group_by1);
    let transactions = filters_by(transactions, |t| {
        t.created_at >= start && t.created_at <= end
    });

    let mut grouped = group_by_date(&transactions, |t| t.created_at, group_by2);

    fill_missing_date_keys(&mut grouped, start, end, group_by2);

    let graph = FinancialGraph::<i64>::from(&grouped, |group| {
        FinancialReport::from(&group.to_vec()).total_profit
    });
    (FinancialReport::from(&transactions), graph)
}

/// Paginate a vector of items
pub fn paginate<T: Clone>(items: &[T], page: i64, per_page: i64) -> PaginatedResult<T> {
    let total_items = items.len() as i64;

    let start = (page.saturating_sub(1)) * per_page;
    let end = (start + per_page).min(total_items);

    let start_usize = start as usize;
    let end_usize = end as usize;

    let page_items = if start < total_items && end > 0 {
        items[start_usize..end_usize].to_vec()
    } else if per_page == -1 {
        items.to_vec()
    } else {
        Vec::new()
    };
    let total_pages = if per_page == -1 {
        1
    } else {
        (total_items as f64 / per_page as f64).ceil() as i64
    };
    PaginatedResult {
        results: page_items,
        page,
        limit: per_page,
        total: total_items,
        total_pages,
    }
}

pub fn get_local_data_path() -> PathBuf {
    let app = APP.get().unwrap();
    let local_path = match app.path().local_data_dir() {
        Ok(val) => val,
        Err(_) => {
            panic!("Could not find local data path");
        }
    };
    local_path
}

pub fn get_or_create_window(
    label: &str,
    url: &str,
    title: &str,
    size: Option<(f64, f64)>,
    resizable: bool,
) -> Result<(bool, WebviewWindow), Error> {
    let t_app = match APP.get() {
        Some(app) => app,
        None => {
            return Err(Error::new(
                "Helper::GetOrCreateWindow",
                "App state not found.",
                get_location!(),
            ));
        }
    };

    let app_handle = t_app.app_handle();

    // Return existing window
    if let Some(window) = app_handle.get_webview_window(label) {
        return Ok((true, window));
    }

    // Build new window
    let mut builder = WebviewWindowBuilder::new(app_handle, label, WebviewUrl::App(url.into()))
        .title(title)
        .resizable(resizable);

    if let Some((w, h)) = size {
        builder = builder.inner_size(w, h);
    }

    let window = builder.build().map_err(|e| {
        Error::new(
            "Helper::GetOrCreateWindow",
            &format!("Failed to build window: {}", e),
            get_location!(),
        )
    })?;
    Ok((false, window))
}

pub async fn populate_item_market_properties(
    properties: &mut Properties,
    raw: impl Into<String>,
    sub_type: Option<SubType>,
    bought: i64,
    list_price: Option<i64>,
    mut operations: OperationSet,
    order_type: OrderType,
    cache: &CacheState,
    wfm: &wf_market::client::Client<Authenticated>,
) -> Result<(), Error> {
    let conn = DATABASE.get().unwrap();
    let raw = raw.into();
    let wfm_sub_type: wf_market::types::SubType = SubTypeExt::from_entity(sub_type.clone());

    // ---------------- Item Info ----------------
    let item_info = cache
        .tradable_item()
        .get_by(&raw)
        .map_err(|e| e.with_location(get_location!()))?;

    properties.set_property_value("name", item_info.name.clone());
    properties.set_property_value("image", item_info.image_url.clone());
    properties.set_property_value("t_type", item_info.sub_type.clone());

    // ---------------- Order Info ----------------
    let order = wfm
        .order()
        .cache_orders()
        .find_order(&item_info.wfm_id, &wfm_sub_type, order_type);

    let (platinum, order_properties) = if let Some(order) = order {
        let order_operations = order
            .properties
            .get_property_value("operations", OperationSet::new());
        operations.merge(&order_operations);
        (order.platinum as i64, order.properties.clone())
    } else {
        (
            list_price.unwrap_or(0),
            wf_market::types::Properties::default(),
        )
    };

    // ---------------- Profitability Info ----------------
    if operations.has("ProfitabilityInfo") {
        let potential_profit = platinum - bought;
        let roi = if bought > 0 {
            (potential_profit as f64 / bought as f64) * 100.0
        } else {
            0.0
        };
        properties.set_property_value("roi_percent", roi);
        properties.set_property_value("potential_profit", potential_profit);
    }
    // ---------------- Transaction Info ----------------
    if operations.has("TransactionInfo") {
        let transactions = TransactionQuery::get_all(
            conn,
            TransactionPaginationQueryDto::new(1, -1)
                .set_wfm_id(&item_info.wfm_id)
                .set_sub_type(sub_type.clone()),
        )
        .await
        .map_err(|e| e.with_location(get_location!()))?;

        properties.set_property_value("report", FinancialReport::from(&transactions.results));
        properties.set_property_value("last_transactions", transactions.take_top(5));
    }

    // ---------------- Market Info ----------------
    if operations.has("MarketInfo") && !operations.has("MarketPopulated") {
        let mut orders = wfm
            .order()
            .get_orders_by_item(&item_info.wfm_url_name)
            .await
            .map_err(|e| {
                Error::from_wfm(
                    "Command::StockItemGetById",
                    "Failed to fetch orders from WFM: {}",
                    e,
                    get_location!(),
                )
            })?;

        orders.filter_by_sub_type(wfm_sub_type.clone(), false);
        orders.filter_user_status(wf_market::enums::StatusType::InGame, false);
        orders.sort_by_platinum();
        orders.apply_item_info(cache)?;

        // Metrics for Highest, Lowest Sell and Buy Prices
        let sell_highest = orders.highest_price(OrderType::Sell);
        let sell_lowest = orders.lowest_price(OrderType::Sell);
        let buy_highest = orders.highest_price(OrderType::Buy);
        let buy_lowest = orders.lowest_price(OrderType::Buy);

        properties.set_property_value("sell_highest_price", sell_highest);
        properties.set_property_value("sell_lowest_price", sell_lowest);
        properties.set_property_value("buy_highest_price", buy_highest);
        properties.set_property_value("buy_lowest_price", buy_lowest);
        properties.set_property_value("supply", orders.sell_orders.len());
        properties.set_property_value("demand", orders.buy_orders.len());

        let spread = sell_lowest - buy_highest;
        properties.set_property_value("spread", spread);

        let spread_pct = if sell_lowest > 0 {
            spread as f64 / sell_lowest as f64 * 100.0
        } else {
            0.0
        };

        properties.set_property_value("spread_percent", spread_pct);
        properties.set_property_value("orders", orders.take_top(5, order_type));
    }
    // ----------------- Market Populated Info -----------------
    if operations.has("MarketPopulated") {
        properties.merge_properties(order_properties.properties, true);
    }
    properties.set_property_value("ui_operations", operations.operations.clone());
    Ok(())
}
pub async fn populate_riven_market_properties(
    properties: &mut Properties,
    raw: impl Into<String>,
    mastery_rank: i64,
    rerolls: i64,
    rank: i32,
    raw_attributes: Vec<(String, f64, bool)>,
    uuid: String,
    bought: i64,
    list_price: Option<i64>,
    mut operations: OperationSet,
    cache: &CacheState,
    wfm: &wf_market::client::Client<Authenticated>,
) -> Result<Vec<RivenAttribute>, Error> {
    let conn = DATABASE.get().unwrap();
    let raw = raw.into();

    // ---------------- Item Info ----------------
    let riven_info = cache
        .riven()
        .get_weapon_by(&raw)
        .map_err(|e| e.with_location(get_location!()))?;

    properties.set_property_value("name", riven_info.name.clone());
    properties.set_property_value("image", riven_info.wfm_icon.clone());
    properties.set_property_value("disposition_rank", riven_info.disposition_rank);

    // ---------------- Attributes Info ----------------
    let mut attributes =
        derive_riven_summary_attributes(&cache, &riven_info, &raw_attributes, rank)?;
    // ---------------- Auction Info ----------------
    let auction = wfm.auction().cache_auctions().get_by_uuid(&uuid);

    let (platinum, auction_properties) = if let Some(auction) = auction {
        let auction_operations = auction
            .properties
            .get_property_value("operations", OperationSet::new());
        operations.merge(&auction_operations);
        (auction.starting_price as i64, auction.properties.clone())
    } else {
        (
            list_price.unwrap_or(0),
            wf_market::types::Properties::default(),
        )
    };

    // ---------------- Profitability Info ----------------
    if operations.has("ProfitabilityInfo") {
        let potential_profit = platinum - bought;
        let roi = if bought > 0 {
            (potential_profit as f64 / bought as f64) * 100.0
        } else {
            0.0
        };
        properties.set_property_value("roi_percent", roi);
        properties.set_property_value("potential_profit", potential_profit);
    }

    // ---------------- Grade Info ----------------
    if operations.has("GradeInfo") {
        let god_roll = &riven_info.god_roll;
        if let Some(god_roll) = god_roll {
            let (grade, grads) = grade_riven(&god_roll, &attributes, "tag");

            for i in 0..grads.len() {
                attributes[i]
                    .properties
                    .set_property_value("grade", grads[i].1.clone());
            }

            properties.set_property_value("grade", grade);
        } else {
            properties.set_property_value("grade", RivenGrade::Unknown);
        }
    }

    // ---------------- Variant Info ----------------
    if operations.has("VariantInfo") {
        let mut weapons = Vec::new();

        let collect_weapon = |weapon: &CacheRivenWeapon| {
            Properties::from(json!({
                "unique_name": weapon.unique_name,
                "name": weapon.name,
                "disposition": weapon.disposition,
                "disposition_rank": weapon.disposition_rank
            }))
        };
        weapons.push(collect_weapon(&riven_info));

        for variant in &riven_info.variants {
            let v = cache.riven().get_weapon_by(&variant.unique_name)?;
            weapons.push(collect_weapon(&v));
        }

        for wea in &mut weapons {
            let disposition = wea.get_property_value("disposition", 0.0);
            let ratio = disposition / riven_info.disposition;

            let ranks = (0..=8)
                .map(|i| scale_attributes(&attributes, ratio, i))
                .collect::<Vec<Vec<RivenAttribute>>>();

            wea.set_property_value("ranks", ranks);
        }

        properties.set_property_value(
            "variants",
            weapons
                .iter()
                .map(|w| w.get_properties(Value::Null))
                .collect::<Vec<Value>>(),
        );
    }

    // ---------------- Roll Evaluation Info ----------------
    if operations.has("RollEvaluation") {
        let roll_evaluation = cache
            .riven()
            .fill_roll_evaluation(&riven_info.unique_name, raw_attributes)?;
        properties.set_property_value("roll_evaluation", roll_evaluation);
    }

    // ---------------- Transaction Info ----------------
    if operations.has("TransactionInfo") {
        let transactions = TransactionQuery::get_all(
            conn,
            TransactionPaginationQueryDto::new(1, -1).set_wfm_id(&riven_info.wfm_id),
        )
        .await
        .map_err(|e| e.with_location(get_location!()))?;

        properties.set_property_value("report", FinancialReport::from(&transactions.results));
        properties.set_property_value("last_transactions", transactions.take_top(5));
    }

    // ---------------- Market Info ----------------
    if operations.has("MarketInfo") && !operations.has("MarketPopulated") {
        let mut filter = wf_market::types::AuctionFilter::new(
            wf_market::enums::AuctionType::Riven,
            &riven_info.wfm_url_name,
        );
        filter.similarity_attributes = Some(
            attributes
                .iter()
                .map(|att| {
                    wf_market::types::ItemAttribute::new(
                        att.url_name.clone(),
                        att.positive,
                        att.value,
                    )
                })
                .collect(),
        );
        filter.similarity = Some(34);

        let mut auctions = wfm.auction().search_auctions(filter).await.map_err(|e| {
            Error::from_wfm(
                "Command::StockRivenGetById",
                "Failed to search auctions",
                e,
                get_location!(),
            )
        })?;
        auctions.sort_by_similarity(false);
        auctions.apply_item_info(&cache)?;

        // Metrics for Lowest Sell
        let sell_highest = auctions.highest_price();
        let sell_lowest = auctions.lowest_price();

        properties.set_property_value("sell_highest_price", sell_highest);
        properties.set_property_value("sell_lowest_price", sell_lowest);
        properties.set_property_value("supply", auctions.total_auctions());

        let mut auctions = auctions.to_vec();
        for auction in auctions.iter_mut().map(|auction| auction.to_auction_mut()) {
            let similarity = auction.item.similarity.clone();
            if let Some(attrs) = &mut auction.item.attributes {
                for attr in attrs.iter_mut() {
                    attr.properties
                        .set_property_value("matched", similarity.has_attribute(&attr.url_name));
                }
            }
        }
        properties.set_property_value("auctions", auctions);
    }

    // ----------------- Endo Info -----------------
    if operations.has("EndoInfo") {
        let endo =
            100 * (mastery_rank - 8) + (22.5 * 2_f64.powi(rank)).floor() as i64 + 200 * rerolls - 7;
        properties.set_property_value("endo", endo);
    }

    // ----------------- Kuva Info -----------------
    if operations.has("KuvaInfo") {
        const COSTS: [i64; 9] = [900, 1000, 1200, 1400, 1700, 2000, 2350, 2750, 3150];

        let kuva = (0..rerolls as usize)
            .map(|i| COSTS.get(i).copied().unwrap_or(3500))
            .sum::<i64>();
        properties.set_property_value("kuva", kuva);
    }

    // ----------------- Market Populated Info -----------------
    if operations.has("MarketPopulated") {
        properties.merge_properties(auction_properties.properties, true);
    }
    properties.set_property_value("attributes", scale_attributes(&attributes, 1.0, rank));
    properties.set_property_value("ui_operations", operations.operations.clone());
    Ok(attributes)
}
