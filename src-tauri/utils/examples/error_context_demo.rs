use serde_json::json;
use utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    init_logger();

    println!("=== Error Context Handling Demo ===\n");

    // Set up logging options
    let log_opts = LoggerOptions {
        console: true,
        file: Some("error_context_demo.log".to_string()),
        show_time: true,
        show_component: true,
        show_elapsed_time: true,
        show_level: true,
        color: true,
    };

    log_info_opt!(
        "Demo",
        log_opts.clone(),
        "Starting error context handling demonstration"
    );

    // 1. Create an error with small context
    log_info_opt!("Demo", log_opts.clone(), "=== Small Context Error ===");
    let small_error = Error::new("Database", "Connection timeout", get_location!())
        .with_cause("Network unreachable")
        .with_context(json!({
            "host": "db.example.com",
            "port": 5432,
            "timeout_ms": 30000,
            "retry_count": 3
        }));

    log_info_opt!(
        "Demo",
        log_opts.clone(),
        "Logging error with small context:"
    );
    small_error.log(Some("error_context_demo.log"));

    // 2. Create an error with large context (over 2048 characters)
    log_info_opt!("Demo", log_opts.clone(), "=== Large Context Error ===");

    // Create a large context object
    let large_context = json!({
        "request_id": "req_1234567890abcdef",
        "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36 Very Long User Agent String That Goes On And On With Lots Of Details About The Browser Version And Platform Information",
        "request_headers": {
            "accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9",
            "accept-encoding": "gzip, deflate, br",
            "accept-language": "en-US,en;q=0.9,es;q=0.8,fr;q=0.7,de;q=0.6,it;q=0.5,pt;q=0.4,ru;q=0.3,ja;q=0.2,ko;q=0.1,zh-CN;q=0.1,zh;q=0.1",
            "cache-control": "no-cache",
            "connection": "keep-alive",
            "cookie": "session_id=abcdefghijklmnopqrstuvwxyz0123456789; preferences=theme_dark_mode_enabled_font_size_large_notifications_all; tracking_consent=accepted_analytics_performance_marketing; user_settings=timezone_UTC_language_en_currency_USD_region_US",
            "host": "api.example.com",
            "pragma": "no-cache",
            "referer": "https://app.example.com/dashboard/analytics/reports/detailed-analysis-with-filters-and-aggregations",
            "sec-fetch-dest": "document",
            "sec-fetch-mode": "navigate",
            "sec-fetch-site": "same-origin",
            "sec-fetch-user": "?1",
            "upgrade-insecure-requests": "1",
            "x-forwarded-for": "192.168.1.100, 10.0.0.50, 172.16.0.25",
            "x-real-ip": "203.0.113.195",
            "x-request-id": "req-uuid-1234-5678-9abc-def012345678"
        },
        "request_body": {
            "query": "SELECT users.id, users.name, users.email, users.created_at, users.last_login, users.status, profiles.bio, profiles.avatar_url, profiles.settings, addresses.street, addresses.city, addresses.state, addresses.country, addresses.postal_code FROM users LEFT JOIN profiles ON users.id = profiles.user_id LEFT JOIN addresses ON users.id = addresses.user_id WHERE users.status = 'active' AND users.created_at > '2024-01-01' AND (profiles.settings LIKE '%premium%' OR users.last_login > '2024-06-01') ORDER BY users.created_at DESC, users.last_login DESC LIMIT 1000 OFFSET 0",
            "parameters": {
                "status_filter": "active",
                "date_from": "2024-01-01T00:00:00Z",
                "premium_filter": true,
                "last_login_threshold": "2024-06-01T00:00:00Z",
                "sort_order": "desc",
                "limit": 1000,
                "offset": 0,
                "include_profiles": true,
                "include_addresses": true,
                "fields": ["id", "name", "email", "created_at", "last_login", "status", "bio", "avatar_url", "settings", "street", "city", "state", "country", "postal_code"]
            }
        },
        "performance_metrics": {
            "query_execution_time_ms": 5432,
            "database_connection_pool_active": 15,
            "database_connection_pool_idle": 5,
            "memory_usage_mb": 256,
            "cpu_usage_percent": 78.5,
            "network_latency_ms": 45,
            "cache_hit_rate_percent": 23.4,
            "index_scan_count": 3,
            "sequential_scan_count": 1,
            "rows_examined": 50000,
            "rows_returned": 1000,
            "temporary_tables_created": 2,
            "sort_operations": 1,
            "join_operations": 2
        },
        "error_details": {
            "error_code": "DB_TIMEOUT_001",
            "error_message": "Query execution timeout after 5000ms",
            "suggested_actions": ["Optimize query indexes", "Increase connection timeout", "Review query complexity", "Consider query caching"],
            "related_errors": ["Previous timeout at 2025-07-26T21:30:15Z", "Connection pool exhaustion at 2025-07-26T21:25:30Z"],
            "stack_trace": [
                "at DatabaseConnection.executeQuery (database.rs:145:23)",
                "at UserService.getActiveUsers (user_service.rs:89:15)",
                "at ApiHandler.handleUserListRequest (api_handler.rs:234:8)",
                "at RequestRouter.route (router.rs:67:12)",
                "at main (main.rs:45:5)"
            ]
        }
    });

    let large_error = Error::new("API", "Database query execution failed", get_location!())
        .with_cause("Query timeout exceeded maximum allowed duration")
        .set_log_level(LogLevel::Error)
        .with_context(large_context);

    log_info_opt!(
        "Demo",
        log_opts.clone(),
        "Logging error with large context (will be truncated for console):"
    );
    large_error.log(Some("error_context_demo.log"));

    // 3. Create an error with massive context
    log_info_opt!("Demo", log_opts.clone(), "=== Massive Context Error ===");

    let massive_data = (0..500)
        .map(|i| format!("data_item_{}: {}", i, "x".repeat(20)))
        .collect::<Vec<_>>();

    let massive_error = Error::new(
        "DataProcessing",
        "Failed to process large dataset",
        get_location!(),
    )
    .with_cause("Memory allocation failure")
    .set_log_level(LogLevel::Critical)
    .with_context(json!({
        "dataset_size": massive_data.len(),
        "sample_data": massive_data,
        "processing_metadata": {
            "start_time": "2025-07-26T21:45:00Z",
            "estimated_completion": "2025-07-26T22:15:00Z",
            "memory_required_gb": 4.5,
            "cpu_cores_utilized": 8,
            "temp_files_created": 15,
            "intermediate_results": format!("{}", "long_intermediate_data_".repeat(100))
        }
    }));

    log_info_opt!(
        "Demo",
        log_opts.clone(),
        "Logging error with massive context (heavily truncated for console):"
    );
    massive_error.log(Some("error_context_demo.log"));

    // 4. Demonstrate error with no context
    log_info_opt!("Demo", log_opts.clone(), "=== Error Without Context ===");
    let simple_error = Error::new("Authentication", "Invalid credentials", get_location!())
        .with_cause("Username not found in database")
        .set_log_level(LogLevel::Warning);

    log_info_opt!(
        "Demo",
        log_opts.clone(),
        "Logging simple error without context:"
    );
    simple_error.log(Some("error_context_demo.log"));

    log_info_opt!(
        "Demo",
        log_opts.clone(),
        "Error context handling demonstration completed"
    );

    println!("\n=== Demo Complete ===");
    println!("Error context handling features demonstrated:");
    println!("• Small contexts (< 2048 chars) - logged in full");
    println!("• Large contexts (> 2048 chars) - truncated for console, full in file");
    println!("• Massive contexts - heavily truncated for console");
    println!("• Errors without context - handled normally");
    println!("\nCheck the log file 'error_context_demo.log' to see full context data.");
    println!("Console output shows truncated versions for readability.");

    Ok(())
}
