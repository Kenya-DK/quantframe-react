# utils - Custom Rust Logging Library

A flexible and feature-rich logging library for Rust applications with colorized output, file logging, and convenient macros.

## Features

- **Multiple Log Levels**: Info, Warning, Error, Debug, Trace, and Critical
- **Colorized Output**: ANSI color codes for different log levels and components
- **File Logging**: Optional file output with automatic log directory creation
- **JSON Logging**: Log structured data directly to JSON files
- **Zip Archive Logging**: Compress logs into zip archives for easy storage and transfer
- **Elapsed Time Tracking**: Track time since logger initialization
- **Flexible Configuration**: Customize what information to display
- **Convenient Macros**: Easy-to-use macros for common logging operations
- **Vec Helper Functions**: Generic functions for finding objects in vectors by key
- **Data Grouping & Analysis**: Powerful grouping functions for time-series and categorical data analysis
- **Log Cleanup**: Automatic cleanup of old log directories
- **Smart Error Context Handling**: Automatic truncation of large context data for console output
- **Comprehensive Error Management**: Rich error types with context, causes, stack traces, and intelligent logging

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
utils = { path = "." }  # or specify version when published
```

Basic usage:

```rust
use utils::*;

fn main() {
    // Initialize the logger (optional, for elapsed time tracking)
    init_logger();

    // Optional: Set custom base path for all logs
    set_base_path(r"C:\Users\Kenya\Desktop\MyLogs");

    // Using macros (recommended)
    log_info!("MyApp", "Application started successfully");
    log_error!("Database", "Failed to connect: {}", "connection timeout");
    log_debug!("Cache", "Cache size: {} items", 42);

    // Using functions directly with custom options
    let custom_opts = LoggerOptions {
        console: true,
        file: Some("app.log".to_string()),
        show_time: true,
        show_component: true,
        show_elapsed_time: true,
        show_level: true,
        color: true,
    };

    info("MyApp", "This will log to both console and file", custom_opts);
}
```

## Configuration Options

The `LoggerOptions` struct allows you to customize the logging behavior:

- `console`: Enable/disable console output
- `file`: Optional file path for log output
- `show_time`: Show timestamp in logs
- `show_component`: Show component name in logs
- `show_elapsed_time`: Show elapsed time since initialization
- `show_level`: Show log level in logs
- `color`: Enable/disable ANSI color codes

## Available Macros

### Logging Macros

- `log_info!(component, message, ...)` - Log info messages
- `log_error!(component, message, ...)` - Log error messages
- `log_warn!(component, message, ...)` - Log warning messages
- `log_debug!(component, message, ...)` - Log debug messages
- `log_critical!(component, message, ...)` - Log critical messages

Each macro also has an `_opt` variant that accepts custom options:

- `log_info_opt!(component, options, message, ...)`

### Utility Macros

- `clear_logs!(days)` - Clear log directories older than specified number of days

### Log Cleanup

You can easily clean up old log directories using the `clear_logs!` macro:

```rust
use utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Clear logs older than 7 days
    clear_logs!(7)?;

    // Clear logs older than 30 days
    clear_logs!(30)?;

    Ok(())
}
```

This function removes entire date-based log directories (e.g., `logs/2025-07-19/`) that are older than the specified number of days. It's particularly useful for automated log cleanup in production systems to prevent disk space issues.

## Vec Helper Functions

The library includes generic helper functions for working with vectors and finding objects by key:

### Available Functions

#### Read-only Functions

- `find_by_key(vec, key, key_extractor)` - Find a single item by key
- `find_all_by_key(vec, key, key_extractor)` - Find all items matching a key
- `find_index_by_key(vec, key, key_extractor)` - Get the index of an item by key

#### Mutable Functions

- `find_by_key_mut(vec, key, key_extractor)` - Find and get mutable reference to item by key
- `find_by_mutable_key(vec, key, key_extractor)` - Find item with mutable key extractor (can modify during search)
- `update_by_key(vec, key, key_extractor, updater)` - Find and update first item matching key
- `update_all_by_key(vec, key, key_extractor, updater)` - Find and update all items matching key
- `apply_to_all_by_mutable_key(vec, key, key_extractor, operation)` - Apply operation to all matching items with mutable key extractor
- `remove_by_key(vec, key, key_extractor)` - Remove first item matching key
- `remove_all_by_key(vec, key, key_extractor)` - Remove all items matching key

#### Multiple Criteria Functions

- `find_by_multiple_criteria(vec, predicate)` - Find item matching multiple conditions
- `find_by_multiple_criteria_mut(vec, predicate)` - Find and get mutable reference to item matching conditions
- `find_all_by_multiple_criteria(vec, predicate)` - Find all items matching multiple conditions
- `find_index_by_multiple_criteria(vec, predicate)` - Get index of item matching multiple conditions
- `update_by_multiple_criteria(vec, predicate, updater)` - Update first item matching multiple conditions
- `update_all_by_multiple_criteria(vec, predicate, updater)` - Update all items matching multiple conditions
- `remove_by_multiple_criteria(vec, predicate)` - Remove first item matching multiple conditions
- `remove_all_by_multiple_criteria(vec, predicate)` - Remove all items matching multiple conditions

### Usage Examples

````rust
use utils::*;

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    access_count: u32,
}

fn main() {
    let mut users = vec![
        User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string(), access_count: 0 },
        User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string(), access_count: 0 },
    ];

    // Find user by ID (read-only)
    if let Some(user) = find_by_key(&users, &1, |u| &u.id) {
        println!("Found user: {}", user.name);
    }

    // Find user by email (read-only)
    let email = "bob@example.com".to_string();
    if let Some(user) = find_by_key(&users, &email, |u| &u.email) {
        println!("Found user by email: {}", user.name);
    }

    // Find user and update access_count during search
    if let Some(user) = find_by_mutable_key(&mut users, &1, |u| {
        u.access_count += 1; // Increment during search
        &u.id
    }) {
        println!("Found and updated user: {}", user.name);
    }

    // Update a specific user
    let updated = update_by_key(&mut users, &2, |u| &u.id, |u| {
        u.name = "Bob Smith".to_string();
    });

    // Update all users with specific criteria
    let count = update_all_by_key(&mut users, &"admin".to_string(), |u| &u.role, |u| {
        u.access_count += 10; // Bonus for admins
    });

    // Remove a user
    if let Some(removed) = remove_by_key(&mut users, &1, |u| &u.id) {
        println!("Removed user: {}", removed.name);
    }

    // Get index of user
    if let Some(index) = find_index_by_key(&users, &2, |u| &u.id) {
        println!("User found at index: {}", index);
    }
}
```

#### Multiple Criteria Examples

```rust
use utils::*;

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    age: u32,
    role: String,
    active: bool,
}

fn main() {
    let mut users = vec![
        User { id: 1, name: "Alice".to_string(), age: 25, role: "admin".to_string(), active: true },
        User { id: 2, name: "Bob".to_string(), age: 20, role: "user".to_string(), active: true },
        User { id: 3, name: "Carol".to_string(), age: 30, role: "admin".to_string(), active: false },
    ];

    // Find user where id = 2 AND age = 20
    if let Some(user) = find_by_multiple_criteria(&users, |u| u.id == 2 && u.age == 20) {
        println!("Found user: {}", user.name);
    }

    // Find all active admins
    let active_admins = find_all_by_multiple_criteria(&users, |u| u.role == "admin" && u.active);
    println!("Found {} active admins", active_admins.len());

    // Update users where age >= 25 AND role = "admin"
    let count = update_all_by_multiple_criteria(&mut users,
        |u| u.age >= 25 && u.role == "admin",
        |u| u.name = format!("{} (Senior)", u.name)
    );

    // Remove inactive users over 25
    let removed = remove_all_by_multiple_criteria(&mut users, |u| !u.active && u.age > 25);

    // Complex conditions with OR logic
    let special_users = find_all_by_multiple_criteria(&users, |u| {
        (u.role == "admin" || u.age < 25) && u.active
    });
}
```These functions work with any type `T` and any comparable key type `K`, making them very flexible for different use cases.

## Text Processing Helpers

The library provides smart text processing utilities for handling large content with intelligent truncation:

### Text Truncation Features

- **Smart Truncation**: Automatically truncate text with informative indicators
- **Dual-Output Processing**: Different truncation rules for console vs file output
- **Configurable Limits**: Customizable length limits and truncation buffers
- **Performance Optimized**: Efficient string processing for large content

### Basic Text Truncation

```rust
use utils::helper::*;

fn main() {
    // Basic truncation with default settings
    let long_text = "x".repeat(5000);
    let (result, was_truncated) = truncate_with_indicator(&long_text, 2048, None);

    if was_truncated {
        println!("Text was truncated: {}", result);
        // Output: "xxxx... [TRUNCATED - 5000 total chars]"
    }

    // Custom truncation buffer
    let (result, was_truncated) = truncate_with_indicator(&long_text, 1024, Some(100));
    println!("Custom buffer result: {}", result);
}
```

### Smart Dual-Output Processing

```rust
use utils::helper::*;

fn main() {
    let large_context = "debug_data".repeat(1000);

    // Process for both console and file with different limits
    let (console_text, file_text) = smart_text_processing(
        &large_context,
        2048,        // Console limit (user-friendly)
        Some(8192),  // File limit (more detailed)
        "Context"    // Label for the content
    );

    // Console gets truncated version for readability
    if let Some(console) = console_text {
        println!("Console: {}", console);
    }

    // File gets more detailed version for debugging
    if let Some(file) = file_text {
        save_to_file(&file); // Full context preserved
    }
}
```

### Integration with Error Handling

These helpers are designed to work seamlessly with the error handling system:

```rust
use utils::helper::*;
use utils::*;
use serde_json::json;

fn log_error_with_smart_processing(error: &Error) {
    let mut base_message = format!("{}", error.message);

    if !error.cause.is_empty() {
        base_message.push_str(&format!(" | Cause: {}", error.cause));
    }

    // Process context with smart truncation
    let (console_context, file_context) = if let Some(context) = &error.context {
        let context_str = context.to_string();
        smart_text_processing(
            &context_str,
            2048,   // MAX_CONTEXT_LENGTH for console
            None,   // No limit for file
            "Context"
        )
    } else {
        (None, None)
    };

    // Process stack trace
    let stack_trace = error.get_stack_trace();
    let (console_stack, file_stack) = smart_text_processing(
        &stack_trace,
        1024,   // MAX_STACK_LENGTH for console
        None,   // No limit for file
        "Stack"
    );

    // Build separate messages for console and file
    let mut console_message = base_message.clone();
    let mut file_message = base_message;

    if let Some(stack) = console_stack {
        console_message.push_str(&stack);
    }
    if let Some(context) = console_context {
        console_message.push_str(&context);
    }

    if let Some(stack) = file_stack {
        file_message.push_str(&stack);
    }
    if let Some(context) = file_context {
        file_message.push_str(&context);
    }

    // Log with appropriate content for each output
    log_to_console(&console_message);
    log_to_file(&file_message);
}
```

### Benefits

- **Console Readability**: Keeps terminal output clean and readable
- **File Completeness**: Preserves full debugging information in log files
- **Performance**: Avoids unnecessary string processing when content is small
- **Flexibility**: Configurable limits for different use cases
- **Consistency**: Standardized truncation indicators across the application

## Error Handling

The library includes a comprehensive error management system with smart context handling and stack trace support:

### Error Features

- **Rich Error Context**: Attach JSON context data to errors for detailed debugging
- **Stack Trace Capture**: Automatic capture of call stack when errors are created (requires RUST_BACKTRACE=1)
- **Smart Context Truncation**: Automatically truncates large context data (>2048 chars) for console output while preserving full context in log files
- **Smart Stack Trace Truncation**: Automatically truncates large stack traces (>1024 chars) for readability
- **Cause Tracking**: Track the underlying cause of errors
- **Severity Levels**: Assign appropriate log levels to errors
- **Sensitive Data Masking**: Automatically mask sensitive information in error context
- **Performance Options**: Option to create errors without stack traces for performance-critical scenarios

### Usage Examples

```rust
use utils::*;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an error with automatic stack trace capture
    let error = Error::new("Database", "Connection failed")
        .with_cause("Network timeout")
        .set_log_level(LogLevel::Error)
        .with_context(json!({
            "host": "db.example.com",
            "port": 5432,
            "timeout_ms": 30000,
            "query": "SELECT * FROM users WHERE active = true"
        }));

    // Log the error (includes stack trace if RUST_BACKTRACE=1)
    error.log(Some("database_errors.log"));

    // Create error without stack trace for performance
    let fast_error = Error::new_without_stack("Cache", "Cache miss")
        .set_log_level(LogLevel::Warning);

    // Enable stack trace manually
    let traced_error = Error::new_without_stack("API", "Rate limit exceeded")
        .with_stack_trace(true)
        .set_log_level(LogLevel::Error);

    // Get stack trace as string for custom handling
    let stack_trace = error.get_stack_trace();
    println!("Stack trace: {}", stack_trace);

    // Create error with large context (will be truncated for console)
    let large_context = json!({
        "large_data": "x".repeat(5000),
        "metadata": "This context is too large for console display"
    });

    let large_error = Error::new("API", "Request processing failed")
        .with_context(large_context);

    // Console shows truncated version, file gets full context
    large_error.log(Some("api_errors.log"));

    // Mask sensitive data
    let mut sensitive_error = Error::new("Auth", "Login failed")
        .with_context(json!({
            "username": "john.doe",
            "password": "secret123",
            "ip_address": "192.168.1.100"
        }));

    sensitive_error.mask_sensitive_data(&["password"]);
    sensitive_error.log(Some("auth_errors.log"));

    Ok(())
}
```

### Stack Trace Features

- **Automatic Capture**: Stack traces are captured automatically when errors are created (requires `RUST_BACKTRACE=1` environment variable)
- **Performance Control**: Use `Error::new_without_stack()` for performance-critical scenarios
- **Manual Control**: Use `.with_stack_trace(true/false)` to explicitly enable or disable
- **Smart Truncation**: Stack traces longer than 1024 characters are truncated for console output
- **Easy Access**: Use `.get_stack_trace()` to retrieve the stack trace as a string

### Context Truncation Rules

- **Small Context** (≤ 2048 chars): Displayed in full on console and file
- **Large Context** (> 2048 chars): Truncated for console with "TRUNCATED" indicator, full context saved to file
- **No Context**: Normal error logging without context data

### Stack Trace Truncation Rules

- **Small Stack** (≤ 1024 chars): Displayed in full
- **Large Stack** (> 1024 chars): Truncated with "STACK TRUNCATED" indicator showing total character count

This ensures console output remains readable while preserving complete debugging information in log files.

## Data Grouping & Analysis

The library provides powerful grouping and analysis functions for organizing data by various criteria, with special support for time-based grouping:

### Grouping Features

- **Generic Grouping**: Group any data by any key using closures
- **Date-Based Grouping**: Sophisticated time-series grouping by year, month, day, or hour
- **Multi-Level Date Keys**: Create hierarchical date keys (e.g., "2025-07-26" or "2025-07-26-14:00")
- **Missing Data Handling**: Automatically fill gaps in time series data
- **Period Boundary Calculation**: Get start/end times for any time period
- **Sorted Results**: Automatically sort grouped data chronologically
- **Flexible Time Zones**: Support for UTC and local time zones

### Basic Grouping Examples

```rust
use utils::grouping::*;
use chrono::{DateTime, Utc, TimeZone};

#[derive(Debug, Clone)]
struct Transaction {
    timestamp: DateTime<Utc>,
    amount: f64,
    category: String,
}

fn main() {
    let transactions = vec![
        Transaction {
            timestamp: Utc.with_ymd_and_hms(2025, 7, 25, 10, 0, 0).unwrap(),
            amount: -45.50,
            category: "Food".to_string(),
        },
        Transaction {
            timestamp: Utc.with_ymd_and_hms(2025, 7, 26, 14, 0, 0).unwrap(),
            amount: 2500.00,
            category: "Income".to_string(),
        },
    ];

    // Group by category
    let by_category = group_by(&transactions, |t| t.category.clone());
    for (category, txns) in &by_category {
        let total: f64 = txns.iter().map(|t| t.amount).sum();
        println!("{}: {} transactions, total: ${:.2}", category, txns.len(), total);
    }

    // Group by day
    let by_day = group_by_date(&transactions, |t| t.timestamp, &[GroupByDate::Year, GroupByDate::Month, GroupByDate::Day]);
    let sorted_days = sort_grouped(&by_day);
    for (day, txns) in sorted_days {
        println!("{}: {} transactions", day, txns.len());
    }
}
```

### Time-Based Grouping

```rust
use utils::grouping::*;
use chrono::{DateTime, Utc, TimeZone};

fn main() {
    let events = vec![/* your time-stamped data */];

    // Group by different time periods
    let by_hour = group_by_date(&events, |e| e.timestamp, &[GroupByDate::Year, GroupByDate::Month, GroupByDate::Day, GroupByDate::Hour]);
    let by_day = group_by_date(&events, |e| e.timestamp, &[GroupByDate::Year, GroupByDate::Month, GroupByDate::Day]);
    let by_month = group_by_date(&events, |e| e.timestamp, &[GroupByDate::Year, GroupByDate::Month]);

    // Fill missing time periods
    let start = Utc.with_ymd_and_hms(2025, 7, 25, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2025, 7, 27, 23, 59, 59).unwrap();
    let mut grouped = group_by_date(&events, |e| e.timestamp, &[GroupByDate::Day]);
    fill_missing_date_keys(&mut grouped, start, end, &[GroupByDate::Day]);

    // Get time period boundaries
    let sample_time = Utc.with_ymd_and_hms(2025, 7, 26, 14, 30, 0).unwrap();
    let (day_start, day_end) = get_start_end_of(sample_time, GroupByDate::Day);
    println!("Day boundaries: {} to {}", day_start, day_end);
}
```

### Advanced Analytics

```rust
use utils::grouping::*;

fn main() {
    let metrics = vec![/* your metrics data */];

    // Multi-dimensional grouping
    let by_service_and_day = group_by(&metrics, |m| {
        let day_key = group_by_date(&[m.clone()], |metric| metric.timestamp, &[GroupByDate::Day])
            .keys().next().unwrap().clone();
        (m.service.clone(), day_key)
    });

    // Performance analysis
    for ((service, day), logs) in &by_service_and_day {
        let avg_response_time: f64 = logs.iter().map(|l| l.response_time_ms as f64).sum::<f64>() / logs.len() as f64;
        let success_rate = logs.iter().filter(|l| l.status_code < 400).count() as f64 / logs.len() as f64 * 100.0;
        println!("{} on {}: {:.1}ms avg, {:.1}% success", service, day, avg_response_time, success_rate);
    }
}
```

### GroupByDate Options

- `GroupByDate::Year` - Group by calendar year
- `GroupByDate::Month` - Group by calendar month
- `GroupByDate::Day` - Group by calendar day
- `GroupByDate::Hour` - Group by hour

These can be combined to create hierarchical keys like:
- `["2025"]` (year only)
- `["2025", "07"]` (year-month)
- `["2025", "07", "26"]` (year-month-day)
- `["2025", "07", "26", "14:00"]` (year-month-day-hour)

## Log Levels and Colors

- **INFO**: Green
- **WARN**: Yellow
- **ERROR**: Red
- **DEBUG**: Blue
- **TRACE**: Cyan
- **CRITICAL**: Red (bold)

## File Logging

When file logging is enabled, logs are automatically saved to date-based subdirectories in the format `logs/YYYY-MM-DD/`. For example:

- `logs/2025-07-26/app.log`
- `logs/2025-07-26/errors.json`
- `logs/2025-07-27/app.log` (next day)

This automatic date organization helps keep logs organized and makes it easy to find logs from specific dates. ANSI color codes are automatically stripped from file output for clean log files.

## Custom Base Path

You can set a custom base directory for all log files using `set_base_path()`:

```rust
use utils::*;

fn main() {
    // Set custom base path (use raw strings for Windows paths)
    set_base_path(r"C:\Users\Kenya\Desktop\MyLogs");

    // All logs will now be saved to:
    // C:\Users\Kenya\Desktop\MyLogs\2025-07-26\filename.log

    log_info!("App", "This goes to the custom directory");
    log_json(json!({"msg": "JSON in custom path"}), "app.json")?;

    // Change base path anytime
    set_base_path("/var/log/myapp");  // Unix-style path
}
````

**Default behavior**: If no base path is set, logs are saved to `logs/` in the current directory.

## JSON Logging

Log structured data directly to JSON files for easy parsing and analysis:

```rust
use utils::*;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Log structured JSON data
    log_json(json!({
        "level": "INFO",
        "component": "App",
        "message": "Application started",
        "timestamp": chrono::Local::now().to_rfc3339(),
        "user_id": 12345,
        "session_id": "sess_abc123"
    }), "app_logs.json")?;

    // Log API request data
    log_json(json!({
        "level": "DEBUG",
        "component": "API",
        "message": "Request processed",
        "endpoint": "/api/v1/users",
        "method": "POST",
        "status_code": 201,
        "response_time_ms": 45.7
    }), "api_logs.json")?;

    Ok(())
}
```

Each JSON log entry is written as a single line to the specified file in the `logs/` directory, making it easy to process with log analysis tools.

## Zip Archive Logging

Create compressed log archives for easy storage, transfer, and organization:

```rust
use utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start a new zip archive
    let zip_logger = ZipLogger::start("application_logs.zip")?;

    // Add individual log entries
    zip_logger.add_log("INFO", "App", "Application started")?;
    zip_logger.add_log("ERROR", "DB", "Connection failed")?;

    // Add existing log files to the archive
    zip_logger.add_log_file("logs/session.log", "session_logs.txt")?;

    // Add custom text files
    zip_logger.add_text_file("System info here", "system_info.txt")?;

    // Finalize the archive
    zip_logger.finalize()?;

    Ok(())
}
```

### Zip Logging Macros

For convenience, use the zip logging macros:

```rust
zip_log_info!(zip_logger, "Component", "Info message");
zip_log_error!(zip_logger, "Component", "Error: {}", error_details);
zip_log_warn!(zip_logger, "Component", "Warning message");
zip_log_debug!(zip_logger, "Component", "Debug info");
zip_log_critical!(zip_logger, "Component", "Critical error!");
```

## Building

```bash
cargo build
cargo test  # Run tests (if any)
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
