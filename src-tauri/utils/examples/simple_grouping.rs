use chrono::{DateTime, TimeZone, Utc};
use utils::grouping::*;

#[derive(Debug, Clone)]
struct Transaction {
    timestamp: DateTime<Utc>,
    amount: f64,
    category: String,
    description: String,
}

fn main() {
    println!("=== Simple Grouping Example ===\n");

    // Sample transaction data
    let transactions = vec![
        Transaction {
            timestamp: Utc.with_ymd_and_hms(2025, 7, 25, 9, 30, 0).unwrap(),
            amount: -45.50,
            category: "Food".to_string(),
            description: "Coffee shop".to_string(),
        },
        Transaction {
            timestamp: Utc.with_ymd_and_hms(2025, 7, 25, 14, 15, 0).unwrap(),
            amount: -120.00,
            category: "Shopping".to_string(),
            description: "Grocery store".to_string(),
        },
        Transaction {
            timestamp: Utc.with_ymd_and_hms(2025, 7, 26, 10, 0, 0).unwrap(),
            amount: 2500.00,
            category: "Income".to_string(),
            description: "Salary".to_string(),
        },
        Transaction {
            timestamp: Utc.with_ymd_and_hms(2025, 7, 26, 16, 45, 0).unwrap(),
            amount: -89.99,
            category: "Shopping".to_string(),
            description: "Online purchase".to_string(),
        },
        Transaction {
            timestamp: Utc.with_ymd_and_hms(2025, 7, 27, 12, 30, 0).unwrap(),
            amount: -25.00,
            category: "Food".to_string(),
            description: "Lunch".to_string(),
        },
    ];

    // Example 1: Group by category
    println!("1. Group by Category:");
    let by_category = group_by(&transactions, |t| t.category.clone());
    for (category, txns) in &by_category {
        let total: f64 = txns.iter().map(|t| t.amount).sum();
        println!(
            "  {}: {} transactions, total: ${:.2}",
            category,
            txns.len(),
            total
        );
    }

    // Example 2: Group by day
    println!("\n2. Group by Day:");
    let by_day = group_by_date(
        &transactions,
        |t| t.timestamp,
        &[GroupByDate::Year, GroupByDate::Month, GroupByDate::Day],
    );
    let sorted_days = sort_grouped(&by_day);
    for (day, txns) in sorted_days {
        let total: f64 = txns.iter().map(|t| t.amount).sum();
        println!(
            "  {}: {} transactions, total: ${:.2}",
            day,
            txns.len(),
            total
        );
    }

    // Example 3: Group by transaction type (income vs expense)
    println!("\n3. Group by Transaction Type:");
    let by_type = group_by(&transactions, |t| {
        if t.amount >= 0.0 { "Income" } else { "Expense" }
    });
    for (txn_type, txns) in &by_type {
        let total: f64 = txns.iter().map(|t| t.amount).sum();
        println!(
            "  {}: {} transactions, total: ${:.2}",
            txn_type,
            txns.len(),
            total
        );
    }

    // Example 4: Time period analysis
    println!("\n4. Time Period Analysis:");
    let sample_time = Utc.with_ymd_and_hms(2025, 7, 26, 14, 30, 0).unwrap();
    let (day_start, day_end) = get_start_end_of(sample_time, GroupByDate::Day);
    println!("  Sample time: {}", sample_time.format("%Y-%m-%d %H:%M:%S"));
    println!(
        "  Day boundaries: {} to {}",
        day_start.format("%Y-%m-%d %H:%M:%S"),
        day_end.format("%Y-%m-%d %H:%M:%S")
    );

    // Find transactions within that day
    let day_transactions: Vec<&Transaction> = transactions
        .iter()
        .filter(|t| t.timestamp >= day_start && t.timestamp <= day_end)
        .collect();

    println!(
        "  Transactions on {}: {}",
        day_start.format("%Y-%m-%d"),
        day_transactions.len()
    );
    for txn in day_transactions {
        println!("    ${:.2} - {}", txn.amount, txn.description);
    }

    println!("\n=== Example Complete ===");
}
