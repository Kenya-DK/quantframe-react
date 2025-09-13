use serde_json::json;
use utils::*;

fn main() -> Result<(), Error> {
    println!("=== Date-Based Logging Demo ===\n");

    // Initialize logger
    init_logger();

    // Regular file logging - will go to logs/2025-07-26/
    let log_opts = LoggerOptions {
        console: true,
        file: Some("daily_app.log".to_string()),
        show_time: true,
        show_component: true,
        show_elapsed_time: true,
        show_level: true,
        color: true,
        ..Default::default()
    };

    log_info_opt!(
        "DateDemo",
        &log_opts.clone(),
        "Regular logging to date-based directory"
    );
    log_warn_opt!(
        "DateDemo",
        &log_opts.clone(),
        "Warning message saved to today's folder"
    );
    log_error_opt!("DateDemo", &log_opts, "Error logged in dated subdirectory");

    // JSON logging - will also go to logs/2025-07-26/
    log_json(
        json!({
            "level": "INFO",
            "component": "DateDemo",
            "message": "JSON logging with date-based directories",
            "timestamp": chrono::Local::now().to_rfc3339(),
            "directory_structure": "logs/YYYY-MM-DD/"
        }),
        "daily_json.json",
    )?;

    let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();

    println!("\n✅ All logs saved to date-based directories!");
    println!("📁 Check logs/{}/", current_date);
    println!("   • daily_app.log (regular log file)");
    println!("   • daily_json.json (JSON formatted log)");
    println!(
        "\n📅 Tomorrow's logs will automatically go to logs/{}/",
        chrono::Local::now()
            .checked_add_days(chrono::Days::new(1))
            .unwrap()
            .format("%Y-%m-%d")
    );

    Ok(())
}
