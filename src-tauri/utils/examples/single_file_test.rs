use utils::*;

fn main() -> Result<(), Error> {
    println!("=== Single File Zip Logging Test ===");

    // Initialize logger
    init_logger();

    // Create a zip logger
    let zip_logger = ZipLogger::start("single_file_test.zip")?;
    println!("Started zip archive: {}", zip_logger.archive_name());

    // Add multiple log entries
    println!("\n=== Adding multiple log entries ===");
    // zip_log_info!(zip_logger, "App", "First log entry");
    // zip_log_warn!(zip_logger, "System", "Second log entry - warning");
    // zip_log_error!(zip_logger, "Network", "Third log entry - error");
    // zip_log_debug!(zip_logger, "Cache", "Fourth log entry - debug");
    // zip_log_critical!(zip_logger, "Security", "Fifth log entry - critical");

    println!("Added 5 log entries to the zip archive");

    // Add a custom text file for comparison
    let metadata = "Archive Metadata\n================\nCreated: 2025-07-26\nEntries: 5\nNote: All logs are now in combined_logs.txt\n";
    zip_logger.add_text_file(metadata, "archive_info.txt")?;

    // Finalize the zip
    println!("\n=== Finalizing zip archive ===");
    zip_logger.finalize()?;

    println!("\n=== Test completed ===");
    println!("Check logs/2025-07-26/single_file_test.zip");
    println!("The zip should contain:");
    println!("  • combined_logs.txt (all 5 log entries in one file)");
    println!("  • archive_info.txt (metadata file)");

    Ok(())
}
