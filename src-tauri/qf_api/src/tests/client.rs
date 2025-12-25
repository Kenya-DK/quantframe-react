use crate::{Client, errors::ApiError};

#[tokio::test]
async fn print_token() {
    let user = "";
    let pass = "";

    assert!(!user.is_empty());
    assert!(!pass.is_empty());

    let mut client = Client::new(
        "N/A",
        "default",
        "v1",
        "https://example.com",
        true,
        "https://example.com",
        "https://example.com",
        "https://example.com",
        "https://example.com",
        "https://example.com",
        false,
    );
    match client.authentication().signin(&user, &pass).await {
        Ok(_) => {
            // client.print_info();
        }
        Err(e) => match e {
            ApiError::InvalidCredentials(err) => {
                println!("Invalid credentials: {}", err);
            }
            _ => {
                println!("Error signing in: {}", e);
            }
        },
    }
    client.set_token("new_token");
}

#[tokio::test]
async fn test_cache_extract() {
    let mut client = Client::new(
        "N/A",
        "default",
        "v1",
        "https://example.com",
        true,
        "https://example.com",
        "https://example.com",
        "https://example.com",
        "https://example.com",
        "https://example.com",
        false,
    );
    match client.cache().download_cache().await {
        Ok(zip_data) => {
            println!("Successfully downloaded cache ({} bytes)", zip_data.len());

            let reader = std::io::Cursor::new(zip_data);
            match zip::ZipArchive::new(reader) {
                Ok(mut archive) => {
                    println!(
                        "Successfully opened zip archive with {} files",
                        archive.len()
                    );

                    let temp_dir = std::env::temp_dir().join("qf_cache_test");

                    // Clear existing test cache
                    if temp_dir.exists() {
                        let _ = std::fs::remove_dir_all(&temp_dir);
                    }

                    let mut total_size = 0u64;
                    for i in 0..archive.len() {
                        if let Ok(mut file) = archive.by_index(i) {
                            let output_path = temp_dir.join(file.mangled_name());

                            if file.is_dir() {
                                let _ = std::fs::create_dir_all(&output_path);
                            } else {
                                if let Some(parent) = output_path.parent() {
                                    if !parent.exists() {
                                        let _ = std::fs::create_dir_all(parent);
                                    }
                                }

                                if let Ok(mut output_file) = std::fs::File::create(&output_path) {
                                    total_size += file.size();
                                    let _ = std::io::copy(&mut file, &mut output_file);
                                }
                            }
                        }
                    }

                    println!("Successfully extracted cache ({} bytes total)", total_size);
                    assert!(total_size > 0, "Should have extracted some data");
                    println!("Extracted to {:?}", temp_dir);
                    // Cleanup
                    // let _ = std::fs::remove_dir_all(&temp_dir);s
                }
                Err(e) => {
                    println!("Failed to read zip archive: {}", e);
                    panic!("Zip extraction failed");
                }
            }
        }
        Err(e) => {
            println!("Failed to download cache: {:?}", e);
            panic!("Cache download failed");
        }
    }
}
