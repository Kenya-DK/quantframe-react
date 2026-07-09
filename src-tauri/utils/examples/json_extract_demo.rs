use serde_json::json;
use std::collections::HashMap;
use utils::*;

fn main() {
    println!("=== JSON Extract Values Demo ===\n");

    let source = json!({
        "live_scraper": {
            "report_to_wfm": true,
            "auto_delete": true,
            "auto_trade": true,
            "stock_mode": "all",
            "trade_modes": ["buy", "sell", "wishlist"],
            "should_delete_other_types": false,
            "stock_item": {
                "blacklist": [],
                "buy_list": [],
                "volume_threshold": 123,
                "profit_threshold": 123,
                "avg_price_cap": 123,
                "trading_tax_cap": 123,
                "max_total_price_cap": 123,
                "price_shift_threshold": 123,
                "buy_quantity": 123,
                "min_wtb_profit_margin": 123,
                "quantity_per_trade": 123,
                "max_stock_quantity": 123,
                "min_sma": 123,
                "min_profit": 123
            },
            "stock_riven": {
                "min_profit": 25,
                "threshold_percentage": 15.0,
                "limit_to": 5,
                "update_interval": 120
            }
        }
    });

    let mut mapping = HashMap::new();
    mapping.insert(
        "live_scraper.report_to_wfm",
        "live_scraper.general.report_to_wfm",
    );
    mapping.insert(
        "live_scraper.auto_delete",
        "live_scraper.general.auto_delete",
    );
    mapping.insert("live_scraper.auto_trade", "live_scraper.general.auto_trade");
    mapping.insert("live_scraper.stock_mode", "live_scraper.general.stock_mode");
    mapping.insert(
        "live_scraper.trade_modes",
        "live_scraper.general.trade_modes",
    );
    mapping.insert(
        "live_scraper.should_delete_other_types",
        "live_scraper.general.delete_conflicting_orders",
    );
    mapping.insert(
        "live_scraper.stock_item.blacklist",
        "live_scraper.items.general.blacklist",
    );
    mapping.insert(
        "live_scraper.stock_item.buy_list",
        "live_scraper.items.general.buy_list",
    );
    mapping.insert(
        "live_scraper.stock_item.volume_threshold",
        "live_scraper.items.wtb.volume_threshold",
    );
    mapping.insert(
        "live_scraper.stock_item.profit_threshold",
        "live_scraper.items.wtb.profit_threshold",
    );
    mapping.insert(
        "live_scraper.stock_item.avg_price_cap",
        "live_scraper.items.wtb.avg_price_cap",
    );
    mapping.insert(
        "live_scraper.stock_item.trading_tax_cap",
        "live_scraper.items.wtb.trading_tax_cap",
    );
    mapping.insert(
        "live_scraper.stock_item.max_total_price_cap",
        "live_scraper.items.wtb.max_total_price_cap",
    );
    mapping.insert(
        "live_scraper.stock_item.price_shift_threshold",
        "live_scraper.items.wtb.price_shift_threshold",
    );
    mapping.insert(
        "live_scraper.stock_item.buy_quantity",
        "live_scraper.items.wtb.buy_quantity",
    );
    mapping.insert(
        "live_scraper.stock_item.min_wtb_profit_margin",
        "live_scraper.items.wtb.min_wtb_profit_margin",
    );
    mapping.insert(
        "live_scraper.stock_item.quantity_per_trade",
        "live_scraper.items.wtb.quantity_per_trade",
    );
    mapping.insert(
        "live_scraper.stock_item.max_stock_quantity",
        "live_scraper.items.wtb.max_stock_quantity",
    );
    mapping.insert(
        "live_scraper.stock_item.min_sma",
        "live_scraper.items.wts.min_sma",
    );
    mapping.insert(
        "live_scraper.stock_item.min_profit",
        "live_scraper.items.wts.min_profit",
    );
    mapping.insert(
        "live_scraper.stock_riven.update_interval",
        "live_scraper.rivens.general.update_interval",
    );
    mapping.insert(
        "live_scraper.stock_riven.min_profit",
        "live_scraper.rivens.wts.min_profit",
    );
    mapping.insert(
        "live_scraper.stock_riven.threshold_percentage",
        "live_scraper.rivens.wts.threshold_percentage",
    );
    mapping.insert(
        "live_scraper.stock_riven.limit_to",
        "live_scraper.rivens.wts.max_results",
    );

    let result = extract_json_values(&source, &mapping);

    println!("Migrated settings:");
    println!("{}", serde_json::to_string_pretty(&result).unwrap());

    println!("\n--- Constraint check ---");
    let riven_min = result["live_scraper"]["rivens"]["wts"]["min_profit"]
        .as_i64()
        .unwrap_or(0);
    let item_min = result["live_scraper"]["items"]["wts"]["min_profit"]
        .as_i64()
        .unwrap_or(0);
    println!(
        "  rivens.wts.min_profit ({}) > items.wts.min_profit ({}) = {}",
        riven_min,
        item_min,
        riven_min > item_min
    );
}
