use utils::*;

fn main() -> Result<(), Error> {
    // Initialize the logger for elapsed time tracking
    init_logger();

    println!("=== utils Zip Logging Demo ===\n");

    // Start a new zip archive
    let zip_logger = ZipLogger::start("application_logs.zip")?;
    println!("Started zip archive: {}", zip_logger.archive_name());

    // Add individual log entries to the zip
    println!("\n=== Adding individual log entries to zip ===");
    zip_logger.add_log("INFO", "App", "Application started successfully")?;
    zip_logger.add_log("WARN", "Database", "Connection pool running low")?;
    zip_logger.add_log("ERROR", "Auth", "Failed login attempt from user 'admin'")?;
    zip_logger.add_log("DEBUG", "Cache", "Cache hit ratio: 85%")?;
    zip_logger.add_log("CRITICAL", "System", "Low disk space warning")?;

    // Also log to console and file simultaneously
    let file_opts = LoggerOptions {
        console: true,
        file: Some("session.log".to_string()),
        show_time: true,
        show_component: true,
        show_elapsed_time: true,
        show_level: true,
        color: true,
    };

    println!("\n=== Logging to console, file, and adding to zip ===");

    // Log some messages normally
    log_info_opt!(
        "ZipDemo",
        file_opts.clone(),
        "This goes to console and file"
    );
    log_error_opt!(
        "ZipDemo",
        file_opts.clone(),
        "Error message in console and file"
    );
    log_debug_opt!("ZipDemo", file_opts, "Debug info in console and file");

    // Add the session log file to the zip archive
    let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let log_file_path = format!("logs/{}/session.log", current_date);
    zip_logger.add_log_file(&log_file_path, "session_logs.txt")?;
    println!("Added session.log to zip archive");

    // Add some custom text files to the zip
    let system_info = "System Information\n=================\nOS: Windows\nMemory: 16GB\nCPU: Intel i7\nDisk: 512GB SSD\n";
    zip_logger.add_text_file(system_info, "system_info.txt")?;

    let error_summary =
        "Error Summary\n============\nTotal Errors: 5\nCritical: 1\nWarnings: 2\nInfo: 10\n";
    zip_logger.add_text_file(error_summary, "error_summary.txt")?;

    println!("Added custom text files to zip archive");

    // Add the existing demo.log to the zip as well
    if std::path::Path::new("logs/demo.log").exists() {
        zip_logger.add_log_file("logs/demo.log", "previous_demo_logs.txt")?;
        println!("Added existing demo.log to zip archive");
    }

    println!("\n=== Finalizing zip archive ===");

    // Finalize the zip archive
    zip_logger.finalize()?;

    println!("\n=== Demo completed ===");
    println!("Check the logs/ directory for:");
    println!("- application_logs.zip (compressed archive)");
    println!("- session.log (regular log file)");
    println!("- demo.log (from previous runs)");

    Ok(())
}
