use utils::*;
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Simplified Folder Management Demo ===");

    // Initialize logger
    init_logger();

    // Old way (complex):
    println!("\n=== Old Complex Way ===");
    println!("// Old complex path building:");
    println!("// let mut path = PathBuf::from(\"logs\");");
    println!("// let date_folder = Local::now().format(\"%Y-%m-%d\").to_string();");
    println!("// path.push(date_folder);");
    println!("// if !path.exists() {{ fs::create_dir_all(&path).unwrap(); }}");
    println!("// path.push(file_name);");

    // New way (simple):
    println!("\n=== New Simple Way ===");
    let folder_path = get_folder();
    println!("let folder_path = get_folder();");
    println!("Folder path: {:?}", folder_path);

    // Demonstrate using it for file creation
    println!("\n=== Creating Files Using get_folder() ===");

    // Create a simple log file
    let log_file_path = folder_path.join("simple_demo.log");
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&log_file_path)?;

    writeln!(
        file,
        "[{}] [INFO] Demo log entry using get_folder()",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    )?;
    writeln!(
        file,
        "[{}] [INFO] File created at: {:?}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        log_file_path
    )?;

    println!("Created log file: {:?}", log_file_path);

    // Test with different base paths
    println!("\n=== Testing with Custom Base Path ===");
    set_base_path("custom_output");

    let custom_folder = get_folder();
    println!("Custom folder path: {:?}", custom_folder);

    let custom_file_path = custom_folder.join("custom_demo.log");
    let mut custom_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&custom_file_path)?;

    writeln!(
        custom_file,
        "[{}] [INFO] Custom path demo",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    )?;

    println!("Created custom file: {:?}", custom_file_path);

    // Reset to default
    set_base_path("logs");

    println!("\n=== Benefits of get_folder() ===");
    println!("✅ Single function call instead of 6+ lines");
    println!("✅ Automatic directory creation");
    println!("✅ Consistent date formatting");
    println!("✅ Respects custom base paths");
    println!("✅ Returns PathBuf for easy file joining");

    println!("\n=== Demo Completed ===");
    println!("Files created:");
    println!("  • {:?}", log_file_path);
    println!("  • {:?}", custom_file_path);

    Ok(())
}
