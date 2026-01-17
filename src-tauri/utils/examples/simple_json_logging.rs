use serde_json::json;
use utils::*;

fn main() -> Result<(), Error> {
    println!("=== Simple JSON Logging Demo ===\n");

    // Initialize logger for elapsed time tracking
    init_logger();

    // Log simple JSON entries
    log_json(
        json!({
            "level": "INFO",
            "component": "App",
            "message": "Application started successfully",
            "timestamp": chrono::Local::now().to_rfc3339()
        }),
        "simple_app.json",
    )?;

    log_json(
        json!({
            "level": "ERROR",
            "component": "Database",
            "message": "Connection failed",
            "timestamp": chrono::Local::now().to_rfc3339(),
            "error_code": "DB_001",
            "retry_count": 3
        }),
        "simple_app.json",
    )?;

    // Log user activity
    log_json(
        json!({
            "level": "INFO",
            "component": "Auth",
            "message": "User login successful",
            "timestamp": chrono::Local::now().to_rfc3339(),
            "user_id": "user_12345",
            "ip_address": "192.168.1.100",
            "session_id": "sess_abc123"
        }),
        "user_activity.json",
    )?;

    // Log API requests
    log_json(
        json!({
            "level": "DEBUG",
            "component": "API",
            "message": "Request processed",
            "timestamp": chrono::Local::now().to_rfc3339(),
            "endpoint": "/api/v1/users",
            "method": "POST",
            "status_code": 201,
            "response_time_ms": 45.7,
            "request_size": 2048
        }),
        "api_logs.json",
    )?;

    // Log system metrics
    log_json(
        json!({
            "level": "WARN",
            "component": "Monitor",
            "message": "High CPU usage detected",
            "timestamp": chrono::Local::now().to_rfc3339(),
            "cpu_percent": 87.5,
            "memory_mb": 1024,
            "disk_usage_percent": 78.2,
            "active_connections": 150
        }),
        "system_metrics.json",
    )?;

    println!("✅ JSON logs written successfully!");
    println!("\nCheck the following files in logs/ directory:");
    println!("- simple_app.json (application logs)");
    println!("- user_activity.json (user authentication logs)");
    println!("- api_logs.json (API request logs)");
    println!("- system_metrics.json (system monitoring logs)");

    Ok(())
}
