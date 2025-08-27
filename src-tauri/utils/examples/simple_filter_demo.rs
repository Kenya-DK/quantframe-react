use utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    init_logger();

    println!("=== Simple Global Filter Demo ===\n");

    println!("1. Logging from multiple components:");
    log_info!("Database", "Connecting to database");
    log_info!("Auth", "Checking user credentials");
    log_info!("Cache", "Loading cache data");
    log_warn!("Database", "Connection slow");

    println!("\n2. Filter to only show 'Database' component:");
    filter_component!("Database");

    log_info!("Database", "This Database log will show");
    log_info!("Auth", "This Auth log will be filtered out");
    log_info!("Cache", "This Cache log will be filtered out");

    println!("\n3. Set minimum log level to Warning:");
    filter_log_level!(LogLevel::Warning);

    log_info!("Database", "This info will be filtered out");
    log_debug!("Database", "This debug will be filtered out");
    log_warn!("Database", "This warning will show");
    log_error!("Database", "This error will show");

    println!("\n=== Demo Completed ===");
    println!("Use filter_component!(\"ComponentName\") to filter by component");
    println!("Use filter_log_level!(LogLevel::Warning) to set minimum log level");

    Ok(())
}
