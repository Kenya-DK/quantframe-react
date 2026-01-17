use utils::*;

fn main() {
    println!("Testing zip logger creation...");

    match ZipLogger::start("test.zip") {
        Ok(zip_logger) => {
            println!(
                "✅ Zip logger created successfully: {}",
                zip_logger.archive_name()
            );

            // Test adding a simple log
            if let Err(e) = zip_logger.add_log("Hello from zip logger!") {
                println!("❌ Error adding log: {}", e);
                return;
            }
            if let Err(e) = zip_logger.add_log("Hello from zip logger!") {
                println!("❌ Error adding log: {}", e);
                return;
            }
            if let Err(e) = zip_logger.add_log("Hello from zip logger!") {
                println!("❌ Error adding log: {}", e);
                return;
            }
            if let Err(e) = zip_logger.add_log("Hello from zip logger!") {
                println!("❌ Error adding log: {}", e);
                return;
            }
            if let Err(e) = zip_logger.add_log("Hello from zip logger!") {
                println!("❌ Error adding log: {}", e);
                return;
            }
            println!("✅ Added log entry to zip");

            // Finalize the zip
            if let Err(e) = zip_logger.finalize() {
                println!("❌ Error finalizing zip: {}", e);
                return;
            }
            println!("✅ Zip finalized successfully");
        }
        Err(e) => {
            println!("❌ Error creating zip logger: {}", e);
        }
    }
}
