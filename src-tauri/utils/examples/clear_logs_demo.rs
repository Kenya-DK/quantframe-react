use utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    init_logger();

    println!("=== Clear Logs Demo ===\n");

    // Set up logging options
    let log_opts = LoggerOptions {
        console: true,
        file: Some("cleanup_demo.log".to_string()),
        show_time: true,
        show_component: true,
        show_elapsed_time: true,
        show_level: true,
        color: true,
        ..Default::default()
    };

    // Log some messages
    log_info_opt!("CleanupDemo", &log_opts.clone(), "Starting cleanup demo...");
    log_info_opt!(
        "CleanupDemo",
        &log_opts.clone(),
        "Current log files before cleanup:"
    );

    // Show current log directory structure
    let logs_path = std::path::Path::new("logs");
    if logs_path.exists() {
        for entry in std::fs::read_dir(logs_path)? {
            let entry = entry?;
            if entry.path().is_dir() {
                if let Some(dir_name) = entry.path().file_name().and_then(|name| name.to_str()) {
                    log_info_opt!(
                        "CleanupDemo",
                        &log_opts.clone(),
                        "Found log directory: {}",
                        dir_name
                    );
                }
            }
        }
    } else {
        log_info_opt!("CleanupDemo", &log_opts.clone(), "No logs directory found");
    }

    // Clear logs older than 7 days
    log_info_opt!(
        "CleanupDemo",
        &log_opts.clone(),
        "Clearing logs older than 7 days..."
    );

    match clear_logs!(7) {
        Ok(()) => {
            log_info_opt!(
                "CleanupDemo",
                &log_opts.clone(),
                "Successfully cleared old logs"
            );
        }
        Err(e) => {
            log_error_opt!(
                "CleanupDemo",
                &log_opts.clone(),
                "Failed to clear logs: {}",
                e
            );
        }
    }

    // Show remaining log directories
    log_info_opt!(
        "CleanupDemo",
        &log_opts.clone(),
        "Remaining log directories after cleanup:"
    );
    if logs_path.exists() {
        for entry in std::fs::read_dir(logs_path)? {
            let entry = entry?;
            if entry.path().is_dir() {
                if let Some(dir_name) = entry.path().file_name().and_then(|name| name.to_str()) {
                    log_info_opt!(
                        "CleanupDemo",
                        &log_opts.clone(),
                        "Remaining directory: {}",
                        dir_name
                    );
                }
            }
        }
    }

    log_info_opt!("CleanupDemo", &log_opts.clone(), "Cleanup demo completed");

    println!("\n=== Demo Complete ===");
    println!("The clear_logs function has been executed.");
    println!("Log directories older than 7 days have been removed.");
    println!(
        "You can change the number of days by calling clear_logs!(x) where x is the number of days to keep."
    );

    Ok(())
}
