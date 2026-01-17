use utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    init_logger();

    println!("=== Global Filter Demo ===\n");

    // Set up basic logging options
    let log_opts = LoggerOptions {
        console: true,
        file: Some("filter_demo.log".to_string()),
        show_time: true,
        show_component: true,
        show_elapsed_time: true,
        show_level: true,
        color: true,
        ..Default::default()
    };

    println!("1. Testing without any filters (all logs should appear):");
    log_info_opt!(
        "Database",
        &log_opts.clone(),
        "Database connection established"
    );
    log_info_opt!("Auth", &log_opts.clone(), "User authentication successful");
    log_warn_opt!(
        "Database",
        &log_opts.clone(),
        "Database query took longer than expected"
    );
    log_debug_opt!("Cache", &log_opts.clone(), "Cache hit for key: user_123");
    log_error_opt!("Auth", &log_opts.clone(), "Invalid login attempt");

    println!("\n2. Setting component filter to 'Database' (only Database logs should appear):");
    set_filter_component("Database");

    log_info_opt!(
        "Database",
        &log_opts.clone(),
        "Database query executed successfully"
    );
    log_info_opt!(
        "Auth",
        &log_opts.clone(),
        "This Auth log should be filtered out"
    );
    log_warn_opt!("Database", &log_opts.clone(), "Database backup started");
    log_debug_opt!(
        "Cache",
        &log_opts.clone(),
        "This Cache log should be filtered out"
    );
    log_error_opt!("Database", &log_opts.clone(), "Database connection timeout");

    println!(
        "\n3. Testing minimum log level filter (setting to Warning - only Warning, Error, Critical should appear):"
    );
    // First clear component filter by creating a new demo without it
    println!("   Note: Component filter is still active, setting min level to Warning...");
    set_min_log_level(LogLevel::Warning);

    log_info_opt!(
        "Database",
        &log_opts.clone(),
        "This info should be filtered out by level"
    );
    log_debug_opt!(
        "Database",
        &log_opts.clone(),
        "This debug should be filtered out by level"
    );
    log_warn_opt!("Database", &log_opts.clone(), "This warning should appear");
    log_error_opt!("Database", &log_opts.clone(), "This error should appear");
    log_critical_opt!("Database", &log_opts.clone(), "This critical should appear");

    println!("\n4. Showing current filter settings:");
    if let Some(component_filter) = get_filter_component() {
        println!("   Active component filter: {}", component_filter);
    } else {
        println!("   No component filter active");
    }

    if let Some(min_level) = get_min_log_level() {
        println!("   Active minimum log level: {:?}", min_level);
    } else {
        println!("   No minimum log level filter active");
    }

    println!("\n=== Global Filter Demo Completed ===");
    println!("Check the filter_demo.log file to see the filtered output!");

    Ok(())
}
