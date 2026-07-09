use serde_json::json;
use utils::merge_json;

fn main() {
    println!("=== merge_json Demo (full source override) ===\n");

    // Example 1: Simple value override
    let mut target = json!({
        "name": "old",
        "version": 1
    });
    let source = json!({
        "name": "new",
        "extra": true
    });

    merge_json(&mut target, &source);
    println!("1. Simple override:");
    println!("   target: {}", target);
    println!("   -> name: {}, extra: {}", target["name"], target["extra"]);
    println!();

    // Example 2: Nested object is fully replaced (not merged)
    let mut target = json!({
        "nested": {
            "a": 1,
            "b": 2,
            "c": 3
        }
    });
    let source = json!({
        "nested": {
            "b": 99
        }
    });

    merge_json(&mut target, &source);
    println!("2. Nested fully replaced:");
    println!("   target: {}", target);
    println!(
        "   -> 'a' and 'c' are lost, only 'b': {}",
        target["nested"]["b"]
    );
    println!();

    // Example 3: Deeply nested override
    let mut target = json!({
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
            "min_profit": 123,
            "threshold_percentage": 123.0,
            "limit_to": 123,
            "update_interval": 123
            }
        }
    });
    let source = json!({
    "live_scraper": {
      "general": {
        "auto_delete": true,
        "auto_trade": true,
        "delete_conflicting_orders": false,
        "report_to_wfm": true,
        "stock_mode": "all",
        "trade_modes": [
          "buy",
          "sell",
          "wishlist"
        ]
      },
      "items": {
        "general": {
          "blacklist": [],
          "buy_list": []
        },
        "wtb": {
          "avg_price_cap": 123,
          "buy_quantity": 123,
          "max_stock_quantity": 123,
          "max_total_price_cap": 123,
          "min_wtb_profit_margin": 123,
          "price_shift_threshold": 123,
          "profit_threshold": 123,
          "quantity_per_trade": 123,
          "trading_tax_cap": 123,
          "volume_threshold": 123
        },
        "wts": {
          "min_profit": 123,
          "min_sma": 123
        }
      },
      "rivens": {
        "general": {
          "update_interval": 123
        },
        "wts": {
          "max_results": 123,
          "min_profit": 123,
          "threshold_percentage": 123.0
        }
      }
    }
      });

    merge_json(&mut target, &source);
    println!("3. Deep nested override:");
    println!("   target: {}", target);
    println!("   -> 'remove' is gone, 'keep' replaced, 'new_key' added");
    println!();

    // Example 4: Merge non-object target with object source
    let mut target = json!("i_am_a_string");
    let source = json!({
        "key": "value"
    });

    merge_json(&mut target, &source);
    println!("4. Non-object target replaced by object source:");
    println!("   target: {}", target);
    println!();

    // Example 5: Empty merge (no source properties)
    let mut target = json!({"existing": true});
    let source = json!({});

    merge_json(&mut target, &source);
    println!("5. Empty source (no changes):");
    println!("   target: {}", target);

    println!("\n=== Demo completed ===");
}
