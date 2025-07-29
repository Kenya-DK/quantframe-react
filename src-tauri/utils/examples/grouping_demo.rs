use chrono::{DateTime, TimeZone, Utc};
use std::collections::HashMap;
use utils::grouping::*;

#[derive(Debug, Clone)]
struct LogEntry {
    id: u32,
    timestamp: DateTime<Utc>,
    level: String,
    message: String,
    component: String,
}

#[derive(Debug, Clone)]
struct SalesRecord {
    id: u32,
    timestamp: DateTime<Utc>,
    amount: f64,
    product: String,
    customer_id: u32,
}

fn main() {
    println!("=== Grouping Examples ===\n");

    // Example 1: Basic grouping by key
    example_basic_grouping();

    // Example 2: Date-based grouping
    example_date_grouping();

    // Example 3: Multi-level date grouping
    example_multilevel_date_grouping();

    // Example 4: Filling missing date keys
    example_fill_missing_dates();

    // Example 5: Get start/end of time periods
    example_start_end_periods();

    // Example 6: Sorting grouped results
    example_sorting_grouped();

    println!("=== All Examples Complete ===");
}

fn example_basic_grouping() {
    println!("1. Basic Grouping by Key:");

    let users = vec![
        ("Alice", "Engineering"),
        ("Bob", "Marketing"),
        ("Charlie", "Engineering"),
        ("Diana", "Sales"),
        ("Eve", "Engineering"),
        ("Frank", "Marketing"),
    ];

    // Group by department
    let grouped_by_dept = group_by(&users, |user| user.1);

    for (dept, employees) in &grouped_by_dept {
        println!(
            "  {}: {:?}",
            dept,
            employees.iter().map(|u| u.0).collect::<Vec<_>>()
        );
    }

    // Group by first letter of name
    let grouped_by_letter = group_by(&users, |user| user.0.chars().next().unwrap());

    println!("\nGrouped by first letter:");
    for (letter, employees) in &grouped_by_letter {
        println!(
            "  {}: {:?}",
            letter,
            employees.iter().map(|u| u.0).collect::<Vec<_>>()
        );
    }

    println!();
}

fn example_date_grouping() {
    println!("2. Date-based Grouping:");

    let log_entries = vec![
        LogEntry {
            id: 1,
            timestamp: Utc.with_ymd_and_hms(2025, 7, 27, 10, 30, 0).unwrap(),
            level: "INFO".to_string(),
            message: "Application started".to_string(),
            component: "Main".to_string(),
        },
        LogEntry {
            id: 2,
            timestamp: Utc.with_ymd_and_hms(2025, 7, 27, 14, 15, 0).unwrap(),
            level: "WARNING".to_string(),
            message: "High memory usage".to_string(),
            component: "Memory".to_string(),
        },
        LogEntry {
            id: 3,
            timestamp: Utc.with_ymd_and_hms(2025, 7, 28, 9, 45, 0).unwrap(),
            level: "ERROR".to_string(),
            message: "Database connection failed".to_string(),
            component: "Database".to_string(),
        },
        LogEntry {
            id: 4,
            timestamp: Utc.with_ymd_and_hms(2025, 7, 28, 11, 20, 0).unwrap(),
            level: "INFO".to_string(),
            message: "User logged in".to_string(),
            component: "Auth".to_string(),
        },
    ];

    // Group by day
    let grouped_by_day = group_by_date(
        &log_entries,
        |entry| entry.timestamp,
        &[GroupByDate::Year, GroupByDate::Month, GroupByDate::Day],
    );

    println!("  Grouped by day:");
    for (date_key, entries) in &grouped_by_day {
        println!("    {}: {} entries", date_key, entries.len());
        for entry in entries {
            println!(
                "      - [{}] {}: {}",
                entry.level, entry.component, entry.message
            );
        }
    }

    // Group by hour
    let grouped_by_hour = group_by_date(
        &log_entries,
        |entry| entry.timestamp,
        &[
            GroupByDate::Year,
            GroupByDate::Month,
            GroupByDate::Day,
            GroupByDate::Hour,
        ],
    );

    println!("\n  Grouped by hour:");
    for (date_key, entries) in &grouped_by_hour {
        println!("    {}: {} entries", date_key, entries.len());
    }

    println!();
}

fn example_multilevel_date_grouping() {
    println!("3. Multi-level Date Grouping:");

    let sales_records = vec![
        SalesRecord {
            id: 1,
            timestamp: Utc.with_ymd_and_hms(2025, 1, 15, 10, 0, 0).unwrap(),
            amount: 100.0,
            product: "Widget A".to_string(),
            customer_id: 1,
        },
        SalesRecord {
            id: 2,
            timestamp: Utc.with_ymd_and_hms(2025, 1, 20, 14, 0, 0).unwrap(),
            amount: 250.0,
            product: "Widget B".to_string(),
            customer_id: 2,
        },
        SalesRecord {
            id: 3,
            timestamp: Utc.with_ymd_and_hms(2025, 2, 5, 9, 0, 0).unwrap(),
            amount: 150.0,
            product: "Widget A".to_string(),
            customer_id: 3,
        },
        SalesRecord {
            id: 4,
            timestamp: Utc.with_ymd_and_hms(2025, 3, 10, 16, 0, 0).unwrap(),
            amount: 300.0,
            product: "Widget C".to_string(),
            customer_id: 1,
        },
        SalesRecord {
            id: 5,
            timestamp: Utc.with_ymd_and_hms(2025, 3, 25, 11, 0, 0).unwrap(),
            amount: 200.0,
            product: "Widget B".to_string(),
            customer_id: 4,
        },
    ];

    // Group by year only
    let grouped_by_year = group_by_date(
        &sales_records,
        |record| record.timestamp,
        &[GroupByDate::Year],
    );
    println!("  Grouped by Year:");
    for (key, records) in &grouped_by_year {
        let total: f64 = records.iter().map(|r| r.amount).sum();
        println!("    {}: {} sales, total: ${:.2}", key, records.len(), total);
    }

    // Group by year and month
    let grouped_by_month = group_by_date(
        &sales_records,
        |record| record.timestamp,
        &[GroupByDate::Year, GroupByDate::Month],
    );
    println!("\n  Grouped by Year-Month:");
    for (key, records) in &grouped_by_month {
        let total: f64 = records.iter().map(|r| r.amount).sum();
        println!("    {}: {} sales, total: ${:.2}", key, records.len(), total);
    }

    // Group by month only (different format)
    let grouped_by_month_only = group_by_date(
        &sales_records,
        |record| record.timestamp,
        &[GroupByDate::Month],
    );
    println!("\n  Grouped by Month only:");
    for (key, records) in &grouped_by_month_only {
        let total: f64 = records.iter().map(|r| r.amount).sum();
        println!(
            "    Month {}: {} sales, total: ${:.2}",
            key,
            records.len(),
            total
        );
    }

    println!();
}

fn example_fill_missing_dates() {
    println!("4. Filling Missing Date Keys:");

    let sales_records = vec![
        SalesRecord {
            id: 1,
            timestamp: Utc.with_ymd_and_hms(2025, 7, 25, 10, 0, 0).unwrap(),
            amount: 100.0,
            product: "Widget A".to_string(),
            customer_id: 1,
        },
        SalesRecord {
            id: 2,
            timestamp: Utc.with_ymd_and_hms(2025, 7, 27, 14, 0, 0).unwrap(),
            amount: 250.0,
            product: "Widget B".to_string(),
            customer_id: 2,
        },
        // Note: July 26 is missing
    ];

    let mut grouped = group_by_date(
        &sales_records,
        |record| record.timestamp,
        &[GroupByDate::Year, GroupByDate::Month, GroupByDate::Day],
    );

    println!("  Before filling missing dates:");
    for (key, records) in &grouped {
        println!("    {}: {} sales", key, records.len());
    }

    // Fill missing dates between July 25 and July 27
    let start = Utc.with_ymd_and_hms(2025, 7, 25, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2025, 7, 27, 23, 59, 59).unwrap();
    fill_missing_date_keys(&mut grouped, start, end, &[GroupByDate::Day]);

    println!("\n  After filling missing dates:");
    let sorted_grouped = sort_grouped(&grouped);
    for (key, records) in sorted_grouped {
        println!("    {}: {} sales", key, records.len());
    }

    println!();
}

fn example_start_end_periods() {
    println!("5. Get Start/End of Time Periods:");

    let sample_date = Utc.with_ymd_and_hms(2025, 7, 27, 14, 30, 45).unwrap();
    println!(
        "  Sample date: {}",
        sample_date.format("%Y-%m-%d %H:%M:%S UTC")
    );

    // Get year boundaries
    let (year_start, year_end) = get_start_end_of(sample_date, GroupByDate::Year);
    println!(
        "  Year boundaries: {} to {}",
        year_start.format("%Y-%m-%d %H:%M:%S"),
        year_end.format("%Y-%m-%d %H:%M:%S")
    );

    // Get month boundaries
    let (month_start, month_end) = get_start_end_of(sample_date, GroupByDate::Month);
    println!(
        "  Month boundaries: {} to {}",
        month_start.format("%Y-%m-%d %H:%M:%S"),
        month_end.format("%Y-%m-%d %H:%M:%S")
    );

    // Get day boundaries
    let (day_start, day_end) = get_start_end_of(sample_date, GroupByDate::Day);
    println!(
        "  Day boundaries: {} to {}",
        day_start.format("%Y-%m-%d %H:%M:%S"),
        day_end.format("%Y-%m-%d %H:%M:%S")
    );

    // Get hour boundaries
    let (hour_start, hour_end) = get_start_end_of(sample_date, GroupByDate::Hour);
    println!(
        "  Hour boundaries: {} to {}",
        hour_start.format("%Y-%m-%d %H:%M:%S"),
        hour_end.format("%Y-%m-%d %H:%M:%S")
    );

    // Demonstrate duration calculation
    println!("\n  Group durations:");
    println!(
        "    Year: {} days",
        GroupByDate::Year.to_duration().num_days()
    );
    println!(
        "    Month: {} days",
        GroupByDate::Month.to_duration().num_days()
    );
    println!(
        "    Day: {} days",
        GroupByDate::Day.to_duration().num_days()
    );
    println!(
        "    Hour: {} hours",
        GroupByDate::Hour.to_duration().num_hours()
    );

    println!();
}

fn example_sorting_grouped() {
    println!("6. Sorting Grouped Results:");

    // Create some data with dates out of order
    let events = vec![
        (
            "Event C",
            Utc.with_ymd_and_hms(2025, 7, 29, 10, 0, 0).unwrap(),
        ),
        (
            "Event A",
            Utc.with_ymd_and_hms(2025, 7, 27, 10, 0, 0).unwrap(),
        ),
        (
            "Event B",
            Utc.with_ymd_and_hms(2025, 7, 28, 10, 0, 0).unwrap(),
        ),
        (
            "Event D",
            Utc.with_ymd_and_hms(2025, 7, 27, 15, 0, 0).unwrap(),
        ),
    ];

    let grouped = group_by_date(
        &events,
        |event| event.1,
        &[GroupByDate::Year, GroupByDate::Month, GroupByDate::Day],
    );

    println!("  Unsorted grouped data:");
    for (key, events) in &grouped {
        println!(
            "    {}: {:?}",
            key,
            events.iter().map(|e| e.0).collect::<Vec<_>>()
        );
    }

    // Sort the grouped data
    let sorted = sort_grouped(&grouped);

    println!("\n  Sorted grouped data:");
    for (key, events) in sorted {
        println!(
            "    {}: {:?}",
            key,
            events.iter().map(|e| e.0).collect::<Vec<_>>()
        );
    }

    // Example with hour-level grouping and sorting
    let detailed_grouped = group_by_date(
        &events,
        |event| event.1,
        &[
            GroupByDate::Year,
            GroupByDate::Month,
            GroupByDate::Day,
            GroupByDate::Hour,
        ],
    );
    let detailed_sorted = sort_grouped(&detailed_grouped);

    println!("\n  Hour-level sorted grouping:");
    for (key, events) in detailed_sorted {
        println!(
            "    {}: {:?}",
            key,
            events.iter().map(|e| e.0).collect::<Vec<_>>()
        );
    }

    println!();
}
