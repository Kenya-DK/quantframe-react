use serde_json::json;
use utils::*;

fn main() -> Result<(), Error> {
    println!("=== Custom Base Path Demo ===\n");

    // Initialize logger
    init_logger();

    // Set a custom base path for all logs
    // Note: On Windows, use forward slashes or double backslashes
    set_base_path(r"C:\Users\Kenya\Desktop\Andet");

    println!("📁 Base path set to: C:\\Users\\Kenya\\Desktop\\Andet");
    println!("📅 All logs will be saved to: C:\\Users\\Kenya\\Desktop\\Andet\\2025-07-26\\");

    // Regular file logging
    let log_opts = LoggerOptions {
        console: true,
        file: Some("custom_app.log".to_string()),
        show_time: true,
        show_component: true,
        show_elapsed_time: true,
        show_level: true,
        color: true,
        ..Default::default()
    };

    log_info_opt!(
        "CustomPath",
        &log_opts.clone(),
        "This log goes to custom base path"
    );
    log_warn_opt!(
        "CustomPath",
        &log_opts.clone(),
        "Warning in custom directory"
    );
    log_error_opt!("CustomPath", &log_opts, "Error logged to custom location");

    // JSON logging
    log_json(
        json!({
            "level": "INFO",
            "component": "CustomPath",
            "message": "JSON log in custom directory",
            "timestamp": chrono::Local::now().to_rfc3339(),
            "base_path": "C:\\Users\\Kenya\\Desktop\\Andet",
            "full_path": "C:\\Users\\Kenya\\Desktop\\Andet\\2025-07-26\\custom_logs.json"
        }),
        "custom_logs.json",
    )?;

    // Zip logging
    let zip_logger = ZipLogger::start("custom_archive.zip")?;
    zip_log_info!(zip_logger, "CustomPath", "Zip archive in custom directory");
    zip_log_warn!(
        zip_logger,
        "CustomPath",
        "All logging respects the custom base path"
    );
    zip_logger.finalize()?;

    println!("\n✅ All logs saved to custom base path!");
    println!("📂 Check the following location:");
    println!("   C:\\Users\\Kenya\\Desktop\\Andet\\2025-07-26\\");
    println!("   • custom_app.log");
    println!("   • custom_logs.json");
    println!("   • custom_archive.zip");

    println!("\n💡 Tip: You can change the base path anytime by calling set_base_path() again!");

    Ok(())
}
