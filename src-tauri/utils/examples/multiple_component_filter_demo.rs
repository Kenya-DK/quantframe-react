use utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    init_logger();

    println!("=== Multiple Component Filter Demo ===\n");

    println!("1. Logging from multiple components (no filters):");
    log_info!("Database", "Database connection established");
    log_info!("Auth", "User authentication successful");
    log_info!("Cache", "Cache initialized");
    log_info!("API", "API server started");
    log_warn!("Network", "Network latency detected");

    println!("\n2. Filter to show only 'Database' component:");
    filter_component!("Database");

    log_info!("Database", "This Database log will show");
    log_info!("Auth", "This Auth log will be filtered out");
    log_info!("Cache", "This Cache log will be filtered out");
    log_warn!("Database", "Database warning will show");

    println!("\n3. Filter to show multiple components: 'Database' and 'Auth':");
    filter_components!("Database", "Auth");

    log_info!("Database", "Database log will show");
    log_info!("Auth", "Auth log will show");
    log_info!("Cache", "Cache log will be filtered out");
    log_info!("API", "API log will be filtered out");
    log_error!("Database", "Database error will show");
    log_error!("Auth", "Auth error will show");

    println!("\n4. Filter to show three components: 'Database', 'Auth', and 'Cache':");
    filter_components!("Database", "Auth", "Cache");

    log_info!("Database", "Database log will show");
    log_info!("Auth", "Auth log will show");
    log_info!("Cache", "Cache log will show");
    log_info!("API", "API log will be filtered out");
    log_info!("Network", "Network log will be filtered out");

    println!("\n5. Add log level filter (Warning and above) with multiple components:");
    filter_log_level!(LogLevel::Warning);

    log_info!("Database", "Database info will be filtered out by level");
    log_debug!("Auth", "Auth debug will be filtered out by level");
    log_warn!("Database", "Database warning will show");
    log_error!("Auth", "Auth error will show");
    log_critical!("Cache", "Cache critical will show");
    log_error!("API", "API error will be filtered out by component");

    println!("\n6. Current filter settings:");
    let components = get_filter_components();
    if !components.is_empty() {
        println!("   Active component filters: {:?}", components);
    } else {
        println!("   No component filters active");
    }

    if let Some(min_level) = get_min_log_level() {
        println!("   Active minimum log level: {:?}", min_level);
    } else {
        println!("   No minimum log level filter active");
    }

    println!("\n=== Multiple Component Filter Demo Completed ===");
    println!("Use filter_component!(\"ComponentName\") for single component");
    println!("Use filter_components!(\"Comp1\", \"Comp2\", \"Comp3\") for multiple components");

    Ok(())
}
