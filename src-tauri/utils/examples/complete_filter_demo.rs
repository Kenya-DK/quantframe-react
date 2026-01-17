use utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    init_logger();

    println!("=== Complete Multiple Component Filter Demo ===\n");

    println!("1. Initial logging (no filters):");
    log_info!("Database", "Database starting");
    log_info!("Auth", "Auth service starting");
    log_info!("Cache", "Cache service starting");
    log_info!("API", "API service starting");

    println!("\n2. Set single component filter:");
    filter_component!("Database");
    log_info!("Database", "Only this will show");
    log_info!("Auth", "This will be filtered");

    println!("\n3. Add another component to the filter:");
    add_filter_component!("Auth");
    log_info!("Database", "Database message will show");
    log_info!("Auth", "Auth message will show");
    log_info!("Cache", "Cache message will be filtered");

    println!("\n4. Set multiple components at once (replaces previous filters):");
    filter_components!("Database", "Auth", "Cache");
    log_info!("Database", "Database shows");
    log_info!("Auth", "Auth shows");
    log_info!("Cache", "Cache shows");
    log_info!("API", "API filtered out");

    println!("\n5. Add log level filtering:");
    filter_log_level!(LogLevel::Warning);
    log_debug!("Database", "Debug filtered by level");
    log_info!("Database", "Info filtered by level");
    log_warn!("Database", "Warning shows");
    log_error!("Auth", "Error shows");
    log_info!("API", "API filtered by component");

    println!("\n6. Current filters:");
    let components = get_filter_components();
    println!("   Components: {:?}", components);
    if let Some(min_level) = get_min_log_level() {
        println!("   Min Level: {:?}", min_level);
    }

    println!("\n7. Clear component filters (keep log level):");
    clear_filter_components!();
    log_warn!("Database", "Database warning shows");
    log_warn!("API", "API warning shows (component filter cleared)");
    log_info!("Database", "Database info filtered by level");

    println!("\n8. Final filters:");
    let components = get_filter_components();
    if components.is_empty() {
        println!("   No component filters active");
    } else {
        println!("   Components: {:?}", components);
    }
    if let Some(min_level) = get_min_log_level() {
        println!("   Min Level: {:?}", min_level);
    }

    println!("\n=== Demo Complete ===");
    println!("Available macros:");
    println!("- filter_component!(\"Name\") - Set single component");
    println!("- filter_components!(\"A\", \"B\", \"C\") - Set multiple components");
    println!("- add_filter_component!(\"Name\") - Add to existing filters");
    println!("- clear_filter_components!() - Clear all component filters");
    println!("- filter_log_level!(LogLevel::Warning) - Set minimum log level");

    Ok(())
}
