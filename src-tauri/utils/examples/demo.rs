use utils::*;

fn main() {
    // Initialize the logger for elapsed time tracking
    init_logger();

    println!("=== utils Logging Library Demo ===\n");

    // Test basic macros
    log_info!("Demo", "Application started successfully");
    log_warn!("Demo", "This is a warning message");
    log_error!("Demo", "This is an error message");
    log_debug!("Demo", "Debug information: value = {}", 42);
    log_critical!("Demo", "Critical system error!");

    // Test with custom options
    let file_opts = LoggerOptions {
        console: true,
        file: Some("demo.log".to_string()),
        show_time: true,
        show_component: true,
        show_elapsed_time: true,
        show_level: true,
        color: true,
        ..Default::default()
    };

    println!("\n=== Testing with file logging ===");
    log_info_opt!(
        "FileDemo",
        &file_opts.clone(),
        "This message will be saved to demo.log"
    );
    log_error_opt!("FileDemo", &file_opts, "Error message also saved to file");

    // Test with no colors
    let no_color_opts = LoggerOptions {
        color: false,
        ..LoggerOptions::default()
    };

    println!("\n=== Testing without colors ===");
    log_info_opt!("NoColor", &no_color_opts, "This message has no colors");

    println!("\n=== Demo completed ===");
}
