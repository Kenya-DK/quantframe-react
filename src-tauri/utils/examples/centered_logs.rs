use utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    init_logger();

    println!("=== Centered Logs Demo ===");

    // Set up logging options
    let log_opts = &LoggerOptions::default().set_file("centered.log");

    info(
        "CenteredLogs",
        "Starting centered logs demo...",
        &log_opts.set_centered(true).set_width(180),
    );

    // Log some messages
    log_info_opt!(
        "CenteredLogs",
        &log_opts.clone(),
        "Starting centered logs demo..."
    );
    log_info_opt!(
        "CenteredLogs",
        &log_opts.clone(),
        "Current log files before cleanup:"
    );
    Ok(())
}
