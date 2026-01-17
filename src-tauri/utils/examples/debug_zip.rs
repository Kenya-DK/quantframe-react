use utils::*;

fn main() -> Result<(), Error> {
    // Initialize logger with default options
    init_logger();

    println!("Debug ZIP Creation Test");

    // Print current working directory
    println!(
        "Current working directory: {:?}",
        std::env::current_dir().unwrap()
    );

    // Print base path
    println!("Base path: {}", get_base_path());

    // Create date folder path
    let date_folder = chrono::Local::now().format("%Y-%m-%d").to_string();
    let base_path = get_base_path();
    let file_path = format!("{}/{}/{}", base_path, date_folder, "debug_archive.zip");

    println!("Attempting to create zip at: {}", file_path);

    // Check if parent directory exists
    if let Some(parent) = std::path::Path::new(&file_path).parent() {
        println!("Parent directory: {:?}", parent);
        println!("Parent exists: {}", parent.exists());

        if !parent.exists() {
            println!("Creating parent directory...");
            std::fs::create_dir_all(parent).unwrap();
            println!("Parent directory created successfully");
        }
    }

    // Try to create the zip logger
    println!("Creating ZipLogger...");
    let zip_logger = ZipLogger::start("debug_archive.zip")?;

    // Finalize
    println!("Finalizing zip...");
    zip_logger.finalize()?;

    // Check if file was created
    let check_path = format!("{}/{}/debug_archive.zip", base_path, date_folder);
    println!("Checking if file exists at: {}", check_path);
    println!(
        "File exists: {}",
        std::path::Path::new(&check_path).exists()
    );

    Ok(())
}
