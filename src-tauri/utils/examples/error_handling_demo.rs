use std::path::PathBuf;

use serde_json::json;
use utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Error Demo ===");

    // Initialize logger
    init_logger();

    // Demo 1: Basic error creation
    println!("\n=== Basic Error Creation ===");
    let basic_error = Error::new(
        "FileSystem",
        "Failed to create log directory",
        get_location!(),
    );
    basic_error.log(Some("error_log.txt"));
    println!("Is critical: {}", basic_error.is_critical());

    // Demo 2: Error with cause
    println!("\n=== Error with Cause ===");
    let error_with_cause = Error::new("Network", "HTTP request failed", get_location!())
        .with_cause("Connection timeout after 30 seconds")
        .set_log_level(LogLevel::Error);
    error_with_cause.log(Some("error_log.txt"));

    // Demo 3: Error with context
    println!("\n=== Error with Context ===");
    let error_with_context = Error::new("Database", "Query execution failed", get_location!())
        .with_cause("Table 'users' doesn't exist")
        .set_log_level(LogLevel::Critical)
        .with_context(json!({
            "query": "SELECT * FROM users WHERE active = true",
            "execution_time_ms": 2500,
            "retry_count": 3,
            "connection_pool_size": 10
        }));
    error_with_context.log(Some("error_log.txt"));

    // Demo 4: Different error levels
    println!("\n=== Different Error Levels ===");
    let warning = Error::new("Cache", "Cache miss for user data", get_location!())
        .set_log_level(LogLevel::Warning)
        .with_context(json!({"user_id": 12345, "cache_key": "user:profile:12345"}));

    let info = Error::new(
        "Monitoring",
        "Performance metric collected",
        get_location!(),
    )
    .set_log_level(LogLevel::Info)
    .with_context(json!({"response_time_ms": 45, "endpoint": "/api/users"}));

    info.log(Some("error_log.txt"));
    warning.log(Some("error_log.txt"));

    // Demo 5: Error conversion from standard errors
    println!("\n=== Error Conversion Examples ===");

    // Simulate an IO error conversion
    let io_error_result: Result<(), std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "Permission denied when writing to log file",
    ));

    if let Err(io_err) = io_error_result {
        let logging_error = Error::from_io(
            "FileLogger",
            &PathBuf::from("/var/log/app.log"),
            "asd",
            io_err,
            get_location!(),
        );
        logging_error.log(Some("error_log.txt"));
    }

    // Demo 6: Using errors with actual logging
    println!("\n=== Logging Errors ===");
    let critical_error = Error::new("Security", "Unauthorized access attempt", get_location!())
        .with_cause("Invalid API key provided")
        .set_log_level(LogLevel::Critical)
        .with_context(json!({
            "ip_address": "192.168.1.100",
            "attempted_endpoint": "/admin/users",
            "timestamp": "2025-07-26T16:10:00Z"
        }));

    // Log the error using regular logging functions
    critical_error.log(Some("error_log.txt"));

    // Demo 7: JSON serialization
    println!("\n=== JSON Serialization ===");
    let json_error = Error::new("API", "Request validation failed", get_location!())
        .with_cause("Missing required field 'email'")
        .set_log_level(LogLevel::Error)
        .with_context(json!({
            "request_body": {"name": "John Doe", "age": 30},
            "validation_errors": ["email is required", "phone format invalid"]
        }));

    let json_string = serde_json::to_string_pretty(&json_error)?;
    println!("Serialized error:\n{}", json_string);

    // Demo 8: Error checking methods
    println!("\n=== Error Checking Methods ===");
    let test_errors = vec![
        Error::new("Test", "Critical issue", get_location!()).set_log_level(LogLevel::Critical),
        Error::new("Test", "Error issue", get_location!()).set_log_level(LogLevel::Error),
        Error::new("Test", "Warning issue", get_location!()).set_log_level(LogLevel::Warning),
        Error::new("Test", "Info message", get_location!()).set_log_level(LogLevel::Info),
    ];

    for error in &test_errors {
        println!(
            "Error: {} | Is Critical: {} | Is Error: {}",
            error.message,
            error.is_critical(),
            error.is_error()
        );
    }

    println!("\n=== Demo Completed ===");
    println!("The Error type provides:");
    println!("✅ Structured error information with component context");
    println!("✅ Flexible error levels (Critical, Error, Warning, Info, Debug, Trace)");
    println!("✅ Optional cause and context information");
    println!("✅ JSON serialization support");
    println!("✅ Conversion from common error types");
    println!("✅ Integration with the logging system");

    Ok(())
}
