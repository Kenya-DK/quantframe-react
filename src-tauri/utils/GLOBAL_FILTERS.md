# Global Logging Filters

The utils library now supports global filtering that applies to all logging operations across your application.

## Component Filtering

Filter logs to only show messages from specific components:

### Single Component Filter

```rust
use utils::*;

// Set up logging
init_logger();

// Log from multiple components
log_info!("Database", "Connection established");
log_info!("Auth", "User logged in");
log_info!("Cache", "Data cached");

// Filter to only show logs from "Database" component
filter_component!("Database");
// or use the function directly:
// set_filter_component("Database");

// Now only Database logs will appear
log_info!("Database", "This will show");
log_info!("Auth", "This will be filtered out");
```

### Multiple Component Filters

```rust
use utils::*;

// Filter to show multiple components at once
filter_components!("Database", "Auth", "Cache");
// or use the function directly:
// set_filter_components(vec!["Database", "Auth", "Cache"]);

// Now only Database, Auth, and Cache logs will appear
log_info!("Database", "This will show");
log_info!("Auth", "This will show");
log_info!("Cache", "This will show");
log_info!("API", "This will be filtered out");
```

### Adding Components to Existing Filters

```rust
use utils::*;

// Start with one component
filter_component!("Database");

// Add more components to the existing filter
add_filter_component!("Auth");
add_filter_component!("Cache");
// or use the function directly:
// add_filter_component("Auth");

// Now Database, Auth, and Cache logs will all show
```

### Clearing Component Filters

```rust
use utils::*;

// Clear all component filters
clear_filter_components!();
// or use the function directly:
// clear_filter_components();

// Now all components will be logged again
```

## Log Level Filtering

Set a minimum log level - only logs at that level or higher will be displayed:

```rust
use utils::*;

// Set minimum log level to Warning
filter_log_level!(LogLevel::Warning);
// or use the function directly:
// set_min_log_level(LogLevel::Warning);

// These will be filtered out (below Warning level)
log_debug!("Component", "Debug message");
log_info!("Component", "Info message");

// These will be displayed (Warning level and above)
log_warn!("Component", "Warning message");
log_error!("Component", "Error message");
log_critical!("Component", "Critical message");
```

## Log Level Priority Order

From lowest to highest priority:

1. `LogLevel::Debug` (priority 0)
2. `LogLevel::Trace` (priority 1)
3. `LogLevel::Info` (priority 2)
4. `LogLevel::Warning` (priority 3)
5. `LogLevel::Error` (priority 4)
6. `LogLevel::Critical` (priority 5)

## Combining Filters

Both component and log level filters can be active simultaneously:

```rust
use utils::*;

// Filter to multiple components AND minimum Warning level
filter_components!("Database", "Auth");
filter_log_level!(LogLevel::Warning);

// Only Database and Auth warnings, errors, and critical messages will show
log_info!("Database", "Filtered out (below Warning)");
log_info!("Cache", "Filtered out (wrong component)");
log_warn!("Database", "This will show");
log_error!("Auth", "This will show");
```

## Checking Current Filters

```rust
use utils::*;

// Check what filters are active
let components = get_filter_components();
if !components.is_empty() {
    println!("Active component filters: {:?}", components);
} else {
    println!("No component filters active");
}

if let Some(min_level) = get_min_log_level() {
    println!("Active minimum log level: {:?}", min_level);
}
```

## Available Functions and Macros

### Functions

- `set_filter_component(component)` - Set single component filter
- `set_filter_components(vec![comp1, comp2])` - Set multiple component filters
- `add_filter_component(component)` - Add component to existing filters
- `get_filter_components()` - Get current component filters (returns Vec<String>)
- `get_filter_component()` - Get first component filter (for backward compatibility)
- `clear_filter_components()` - Clear all component filters
- `set_min_log_level(level)` - Set global minimum log level
- `get_min_log_level()` - Get current minimum log level

### Convenience Macros

- `filter_component!("ComponentName")` - Set single component filter
- `filter_components!("Comp1", "Comp2", "Comp3")` - Set multiple component filters
- `add_filter_component!("ComponentName")` - Add component to existing filters
- `clear_filter_components!()` - Clear all component filters
- `filter_log_level!(LogLevel::Warning)` - Set minimum log level

## Complete Example

```rust
use utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();

    // Initial logging - all components show
    log_info!("Database", "Starting");
    log_info!("Auth", "Starting");
    log_info!("Cache", "Starting");

    // Filter to single component
    filter_component!("Database");
    log_info!("Database", "Shows");
    log_info!("Auth", "Filtered");

    // Add more components
    add_filter_component!("Auth");
    log_info!("Database", "Shows");
    log_info!("Auth", "Shows");
    log_info!("Cache", "Filtered");

    // Set multiple components (replaces previous)
    filter_components!("Database", "Auth", "Cache");

    // Add log level filtering
    filter_log_level!(LogLevel::Warning);
    log_info!("Database", "Filtered by level");
    log_warn!("Database", "Shows");

    // Clear component filters (keep log level)
    clear_filter_components!();
    log_warn!("API", "Shows (component filter cleared)");

    Ok(())
}
```

## Notes

- Filters are **global** and apply to all logging operations
- Component filtering is case-sensitive exact match
- Multiple component filters work with OR logic (any matching component shows)
- Component and log level filters work together with AND logic
- Filters persist for the lifetime of the application
- Both console and file logging respect the global filters
- Thread-safe: Uses RwLock internally for component filters
