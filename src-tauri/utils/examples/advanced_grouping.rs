use chrono::{DateTime, TimeZone, Utc};
use std::collections::HashMap;
use utils::grouping::*;

#[derive(Debug, Clone)]
struct LogMetric {
    timestamp: DateTime<Utc>,
    service: String,
    level: String,
    response_time_ms: u32,
    status_code: u16,
    endpoint: String,
}

fn main() {
    println!("=== Advanced Grouping Examples ===\n");

    // Sample log metrics data
    let log_metrics = create_sample_metrics();

    // Example 1: Multi-dimensional grouping
    example_multi_dimensional_grouping(&log_metrics);

    // Example 2: Time series analysis with missing data
    example_time_series_with_gaps(&log_metrics);

    // Example 3: Performance analysis by time periods
    example_performance_analysis(&log_metrics);

    // Example 4: Custom aggregations
    example_custom_aggregations(&log_metrics);

    println!("=== Advanced Examples Complete ===");
}

fn create_sample_metrics() -> Vec<LogMetric> {
    vec![
        LogMetric {
            timestamp: Utc.with_ymd_and_hms(2025, 7, 25, 8, 0, 0).unwrap(),
            service: "api".to_string(),
            level: "INFO".to_string(),
            response_time_ms: 120,
            status_code: 200,
            endpoint: "/users".to_string(),
        },
        LogMetric {
            timestamp: Utc.with_ymd_and_hms(2025, 7, 25, 8, 15, 0).unwrap(),
            service: "api".to_string(),
            level: "ERROR".to_string(),
            response_time_ms: 5000,
            status_code: 500,
            endpoint: "/orders".to_string(),
        },
        LogMetric {
            timestamp: Utc.with_ymd_and_hms(2025, 7, 25, 9, 30, 0).unwrap(),
            service: "web".to_string(),
            level: "INFO".to_string(),
            response_time_ms: 80,
            status_code: 200,
            endpoint: "/dashboard".to_string(),
        },
        LogMetric {
            timestamp: Utc.with_ymd_and_hms(2025, 7, 25, 10, 45, 0).unwrap(),
            service: "api".to_string(),
            level: "WARN".to_string(),
            response_time_ms: 2500,
            status_code: 429,
            endpoint: "/users".to_string(),
        },
        // July 26 - morning gap
        LogMetric {
            timestamp: Utc.with_ymd_and_hms(2025, 7, 26, 14, 0, 0).unwrap(),
            service: "api".to_string(),
            level: "INFO".to_string(),
            response_time_ms: 95,
            status_code: 200,
            endpoint: "/orders".to_string(),
        },
        LogMetric {
            timestamp: Utc.with_ymd_and_hms(2025, 7, 26, 15, 30, 0).unwrap(),
            service: "web".to_string(),
            level: "INFO".to_string(),
            response_time_ms: 110,
            status_code: 200,
            endpoint: "/login".to_string(),
        },
        LogMetric {
            timestamp: Utc.with_ymd_and_hms(2025, 7, 27, 9, 0, 0).unwrap(),
            service: "api".to_string(),
            level: "ERROR".to_string(),
            response_time_ms: 8000,
            status_code: 503,
            endpoint: "/orders".to_string(),
        },
        LogMetric {
            timestamp: Utc.with_ymd_and_hms(2025, 7, 27, 11, 30, 0).unwrap(),
            service: "web".to_string(),
            level: "INFO".to_string(),
            response_time_ms: 150,
            status_code: 200,
            endpoint: "/profile".to_string(),
        },
    ]
}

fn example_multi_dimensional_grouping(metrics: &[LogMetric]) {
    println!("1. Multi-dimensional Grouping:");

    // Group by service and log level
    let by_service_level = group_by(metrics, |m| (m.service.clone(), m.level.clone()));

    println!("  By Service and Level:");
    for ((service, level), logs) in &by_service_level {
        let avg_response_time: f64 =
            logs.iter().map(|l| l.response_time_ms as f64).sum::<f64>() / logs.len() as f64;
        println!(
            "    {}/{}: {} logs, avg response: {:.1}ms",
            service,
            level,
            logs.len(),
            avg_response_time
        );
    }

    // Group by status code category
    let by_status_category = group_by(metrics, |m| match m.status_code {
        200..=299 => "Success",
        400..=499 => "Client Error",
        500..=599 => "Server Error",
        _ => "Other",
    });

    println!("\n  By Status Category:");
    for (category, logs) in &by_status_category {
        println!("    {}: {} requests", category, logs.len());
    }

    // Group by endpoint and day
    let by_endpoint_day = group_by(metrics, |m| {
        let day_key = group_by_date(
            &[m.clone()],
            |metric| metric.timestamp,
            &[GroupByDate::Year, GroupByDate::Month, GroupByDate::Day],
        )
        .keys()
        .next()
        .unwrap()
        .clone();
        (m.endpoint.clone(), day_key)
    });

    println!("\n  By Endpoint and Day:");
    for ((endpoint, day), logs) in &by_endpoint_day {
        println!("    {} on {}: {} requests", endpoint, day, logs.len());
    }

    println!();
}

fn example_time_series_with_gaps(metrics: &[LogMetric]) {
    println!("2. Time Series Analysis with Gap Filling:");

    // Group by hour
    let mut by_hour = group_by_date(
        metrics,
        |m| m.timestamp,
        &[
            GroupByDate::Year,
            GroupByDate::Month,
            GroupByDate::Day,
            GroupByDate::Hour,
        ],
    );

    println!("  Original hourly data:");
    let sorted_hours = sort_grouped(&by_hour);
    for (hour, logs) in &sorted_hours {
        println!("    {}: {} logs", hour, logs.len());
    }

    // Fill missing hours
    let start = Utc.with_ymd_and_hms(2025, 7, 25, 8, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2025, 7, 27, 12, 0, 0).unwrap();
    fill_missing_date_keys(&mut by_hour, start, end, &[GroupByDate::Hour]);

    println!("\n  After filling gaps:");
    let filled_sorted = sort_grouped(&by_hour);
    for (hour, logs) in filled_sorted {
        if logs.is_empty() {
            println!("    {}: 0 logs [FILLED]", hour);
        } else {
            println!("    {}: {} logs", hour, logs.len());
        }
    }

    println!();
}

fn example_performance_analysis(metrics: &[LogMetric]) {
    println!("3. Performance Analysis by Time Periods:");

    // Analyze performance by day
    let by_day = group_by_date(
        metrics,
        |m| m.timestamp,
        &[GroupByDate::Year, GroupByDate::Month, GroupByDate::Day],
    );
    let sorted_days = sort_grouped(&by_day);

    println!("  Daily Performance Summary:");
    for (day, logs) in sorted_days {
        if logs.is_empty() {
            continue;
        }

        let total_requests = logs.len();
        let avg_response_time: f64 =
            logs.iter().map(|l| l.response_time_ms as f64).sum::<f64>() / total_requests as f64;
        let max_response_time = logs.iter().map(|l| l.response_time_ms).max().unwrap();
        let min_response_time = logs.iter().map(|l| l.response_time_ms).min().unwrap();
        let error_count = logs.iter().filter(|l| l.status_code >= 400).count();
        let error_rate = (error_count as f64 / total_requests as f64) * 100.0;

        println!("    {}:", day);
        println!("      Requests: {}", total_requests);
        println!("      Avg Response: {:.1}ms", avg_response_time);
        println!(
            "      Min/Max Response: {}ms/{}ms",
            min_response_time, max_response_time
        );
        println!(
            "      Error Rate: {:.1}% ({} errors)",
            error_rate, error_count
        );
    }

    // Time boundary analysis
    println!("\n  Time Boundary Analysis:");
    let sample_time = metrics[0].timestamp;
    let boundaries = [
        ("Hour", get_start_end_of(sample_time, GroupByDate::Hour)),
        ("Day", get_start_end_of(sample_time, GroupByDate::Day)),
        ("Month", get_start_end_of(sample_time, GroupByDate::Month)),
        ("Year", get_start_end_of(sample_time, GroupByDate::Year)),
    ];

    for (period, (start, end)) in boundaries {
        let metrics_in_period: Vec<&LogMetric> = metrics
            .iter()
            .filter(|m| m.timestamp >= start && m.timestamp <= end)
            .collect();

        println!(
            "    {} period ({} to {}): {} metrics",
            period,
            start.format("%Y-%m-%d %H:%M"),
            end.format("%Y-%m-%d %H:%M"),
            metrics_in_period.len()
        );
    }

    println!();
}

fn example_custom_aggregations(metrics: &[LogMetric]) {
    println!("4. Custom Aggregations:");

    // Service health score by day
    let by_service_day = group_by(metrics, |m| {
        let day_key = group_by_date(
            &[m.clone()],
            |metric| metric.timestamp,
            &[GroupByDate::Year, GroupByDate::Month, GroupByDate::Day],
        )
        .keys()
        .next()
        .unwrap()
        .clone();
        (m.service.clone(), day_key)
    });

    println!("  Service Health Scores (by day):");
    for ((service, day), logs) in &by_service_day {
        let total_requests = logs.len() as f64;
        let successful_requests = logs.iter().filter(|l| l.status_code < 400).count() as f64;
        let avg_response_time: f64 =
            logs.iter().map(|l| l.response_time_ms as f64).sum::<f64>() / total_requests;

        // Custom health score: success rate - response time penalty
        let success_rate = (successful_requests / total_requests) * 100.0;
        let response_penalty = (avg_response_time / 1000.0).min(50.0); // Cap at 50 point penalty
        let health_score = (success_rate - response_penalty).max(0.0);

        println!(
            "    {} on {}: Health Score: {:.1}/100 (Success: {:.1}%, Avg RT: {:.0}ms)",
            service, day, health_score, success_rate, avg_response_time
        );
    }

    // Peak usage hours
    let by_hour_only = group_by_date(metrics, |m| m.timestamp, &[GroupByDate::Hour]);
    let mut hour_counts: Vec<(String, usize)> = by_hour_only
        .iter()
        .map(|(hour, logs)| (hour.clone(), logs.len()))
        .collect();
    hour_counts.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count descending

    println!("\n  Peak Usage Hours:");
    for (i, (hour, count)) in hour_counts.iter().take(3).enumerate() {
        println!("    {}. Hour {}: {} requests", i + 1, hour, count);
    }

    // Endpoint performance ranking
    let by_endpoint = group_by(metrics, |m| m.endpoint.clone());
    let mut endpoint_performance: Vec<(String, f64, usize)> = by_endpoint
        .iter()
        .map(|(endpoint, logs)| {
            let avg_response: f64 =
                logs.iter().map(|l| l.response_time_ms as f64).sum::<f64>() / logs.len() as f64;
            (endpoint.clone(), avg_response, logs.len())
        })
        .collect();
    endpoint_performance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap()); // Sort by avg response time

    println!("\n  Endpoint Performance Ranking (by avg response time):");
    for (i, (endpoint, avg_response, count)) in endpoint_performance.iter().enumerate() {
        println!(
            "    {}. {}: {:.1}ms avg ({} requests)",
            i + 1,
            endpoint,
            avg_response,
            count
        );
    }

    println!();
}
