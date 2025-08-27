use utils::*;

fn main() -> Result<(), Error> {
    // Initialize the logger
    init_logger();

    println!("=== Combined Logging Example ===\n");

    // Set up regular file logging
    let log_opts = LoggerOptions {
        console: true,
        file: Some("app_session.log".to_string()),
        show_time: true,
        show_component: true,
        show_elapsed_time: true,
        show_level: true,
        color: true,
        ..Default::default()
    };

    // Start a zip archive for this session
    let zip_logger = ZipLogger::start("session_archive.zip")?;

    // Simulate some application activity
    log_info_opt!("App", &log_opts.clone(), "Application starting up...");
    zip_log_info!(zip_logger, "App", "Application starting up...");

    log_info_opt!("Config", &log_opts.clone(), "Loading configuration files");
    zip_log_info!(zip_logger, "Config", "Loading configuration files");

    log_warn_opt!("Memory", &log_opts.clone(), "Memory usage at 75%");
    zip_log_warn!(zip_logger, "Memory", "Memory usage at 75%");

    log_error_opt!(
        "Network",
        &log_opts.clone(),
        "Failed to connect to external API"
    );
    zip_log_error!(zip_logger, "Network", "Failed to connect to external API");

    log_debug_opt!("Cache", &log_opts.clone(), "Cache miss ratio: {:.2}%", 15.5);
    zip_log_debug!(zip_logger, "Cache", "Cache miss ratio: 15.50%");

    log_critical_opt!(
        "Security",
        &log_opts.clone(),
        "Multiple failed login attempts detected"
    );
    zip_log_critical!(
        zip_logger,
        "Security",
        "Multiple failed login attempts detected"
    );

    log_info_opt!(
        "App",
        &log_opts.clone(),
        "Application shutting down gracefully"
    );
    zip_log_info!(zip_logger, "App", "Application shutting down gracefully");

    // Add the session log file to the zip archive
    let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let log_file_path = format!("logs/{}/app_session.log", current_date);
    zip_logger.add_log_file(&log_file_path, "complete_session.log")?;

    // Add some metadata
    let metadata = format!(
        "Session Metadata\n================\nStart Time: {}\nTotal Log Entries: 7\nWarnings: 1\nErrors: 1\nCritical: 1\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    zip_logger.add_text_file(metadata, "session_metadata.txt")?;

    // Finalize the zip archive
    zip_logger.finalize()?;

    // Clean up old logs (keep logs from last 30 days)
    println!("\n=== Cleaning up old logs ===");
    match clear_logs!(30) {
        Ok(()) => println!("Successfully cleaned up logs older than 30 days"),
        Err(e) => println!("Failed to clean up logs: {}", e),
    }

    println!("\n=== Session Complete ===");
    let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    println!(
        "Regular logs saved to: logs/{}/app_session.log",
        current_date
    );
    println!(
        "Compressed archive created: logs/{}/session_archive.zip",
        current_date
    );
    println!("Archive contains:");
    println!("  • Individual timestamped log entries");
    println!("  • Complete session log file");
    println!("  • Session metadata");

    Ok(())
}
