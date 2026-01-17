use std::fs;
use utils::{Error, zip_folder::ZipOptions};

fn main() -> Result<(), Error> {
    println!("=== ZipOptions Standalone API Demo ===");

    // Create demo files for testing
    fs::create_dir_all("demo_folder/temp")?;
    fs::create_dir_all("demo_folder/logs")?;
    fs::write(
        "demo_folder/config.json",
        r#"{"username": "admin", "password": "secret123", "api_key": "abc123"}"#,
    )?;
    fs::write("demo_folder/readme.txt", "This is a readme file.")?;
    fs::write("demo_folder/temp/cache.tmp", "Temporary cache data")?;
    fs::write("demo_folder/logs/app.log", "Log entry 1\nLog entry 2")?;

    println!("Demo files created. Testing different ZIP approaches:");

    // 1. Simple ZIP using ZipOptions directly
    ZipOptions::new().create_zip("demo_folder", "standalone_simple.zip")?;
    println!("1. ✓ Simple ZIP created using ZipOptions::new().create_zip()");

    // 2. ZIP with exclusions using ZipOptions
    ZipOptions::new()
        .exclude_patterns(&["*.log", "temp/"])
        .create_zip("demo_folder", "standalone_clean.zip")?;
    println!("2. ✓ Clean ZIP created (no logs/temp) using fluent API");

    // 3. ZIP with password masking using ZipOptions
    ZipOptions::new()
        .mask_properties(&["password", "api_key"])
        .create_zip("demo_folder", "standalone_secure.zip")?;
    println!("3. ✓ Secure ZIP created (passwords masked) using fluent API");

    // 4. Production-ready ZIP with all options using ZipOptions
    ZipOptions::new()
        .exclude_patterns(&["*.log", "temp/"])
        .mask_properties(&["password", "api_key"])
        .include_hidden(false)
        .create_zip("demo_folder", "standalone_production.zip")?;
    println!("4. ✓ Production ZIP created (clean + secure) using fluent API");

    // Show file sizes
    let simple_size = fs::metadata("standalone_simple.zip")?.len();
    let clean_size = fs::metadata("standalone_clean.zip")?.len();
    let secure_size = fs::metadata("standalone_secure.zip")?.len();
    let production_size = fs::metadata("standalone_production.zip")?.len();

    println!("\nFile sizes:");
    println!("   Simple: {} bytes", simple_size);
    println!("   Clean: {} bytes", clean_size);
    println!("   Secure: {} bytes", secure_size);
    println!("   Production: {} bytes", production_size);

    // Clean up
    fs::remove_dir_all("demo_folder").ok();
    fs::remove_file("standalone_simple.zip").ok();
    fs::remove_file("standalone_clean.zip").ok();
    fs::remove_file("standalone_secure.zip").ok();
    fs::remove_file("standalone_production.zip").ok();

    println!("\n=== ZipOptions standalone API works perfectly! ===");
    println!(
        "Now you can use ZipOptions::new().create_zip() instead of Error::zip_folder_with_options()"
    );

    Ok(())
}
