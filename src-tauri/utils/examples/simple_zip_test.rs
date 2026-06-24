use utils::*;

fn main() {
    println!("Testing zip logger creation...");

    let zip_logger = ZipLogger::new();
    println!("✅ Zip logger created successfully: test.zip");

    // Test adding a simple log
    zip_logger.add_log("Hello from zip logger!");
    zip_logger.add_log("Hello from zip logger!");
    zip_logger.add_log("Hello from zip logger!");
    zip_logger.add_log("Hello from zip logger!");
    zip_logger.add_log("Hello from zip logger!");
    println!("✅ Added log entry to zip");

    // Finalize the zip
    if let Err(e) = zip_logger.finalize("test.zip") {
        println!("❌ Error finalizing zip: {}", e);
        return;
    }
    println!("✅ Zip finalized successfully");
}
