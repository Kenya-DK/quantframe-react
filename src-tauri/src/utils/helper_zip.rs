#[derive(Clone, Debug)]
pub struct ZipEntry {
    pub file_path: PathBuf,
    pub sub_path: Option<String>,
    pub include_dir: bool,
}

pub fn get_zip_entrys(path: PathBuf, in_subfolders: bool) -> Result<Vec<ZipEntry>, AppError> {
    let mut files: Vec<ZipEntry> = Vec::new();
    for path in fs::read_dir(path).unwrap() {
        let path = path.unwrap().path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            let subfiles = get_zip_entrys(path.to_owned(), in_subfolders)?;
            for mut subfile in subfiles {
                let sub_path = subfile.sub_path.clone().unwrap_or("".to_string());
                // Remove the first slash if it exists
                let full_path = format!("{}/{}", dir_name, sub_path);
                subfile.sub_path = Some(full_path);
                files.push(subfile);
            }
        }
        if path.is_file() {
            files.push(ZipEntry {
                file_path: path.clone(),
                sub_path: None,
                include_dir: false,
            });
        }
    }
    Ok(files)
}

pub fn create_zip_file(mut files: Vec<ZipEntry>, zip_path: &str) -> Result<(), AppError> {
    let zip_file_path = Path::new(&zip_path);
    let zip_file =
        File::create(&zip_file_path).map_err(|e| AppError::new("Zip", eyre!(e.to_string())))?;
    let mut zip = ZipWriter::new(zip_file);

    // Get all files that are directories and add them to the files list
    let mut files_to_compress: Vec<ZipEntry> = Vec::new();

    for file_entry in &files {
        if file_entry.include_dir {
            let subfiles = get_zip_entrys(file_entry.file_path.clone(), true)?;
            for mut subfile in subfiles {
                if subfile.sub_path.is_some() {
                    subfile.sub_path = Some(format!(
                        "{}/{}",
                        file_entry.sub_path.clone().unwrap_or("".to_string()),
                        subfile.sub_path.clone().unwrap_or("".to_string())
                    ));
                }
                files_to_compress.push(subfile);
            }
        }
    }
    files.append(&mut files_to_compress);

    // Set compression options (e.g., compression method)
    let options = FileOptions::default().compression_method(CompressionMethod::DEFLATE);

    for file_entry in &files {
        if file_entry.include_dir {
            continue;
        }

        let file_path = Path::new(&file_entry.file_path)
            .canonicalize()
            .map_err(|e| AppError::new("Zip", eyre!(e.to_string())))?;

        if !file_path.exists() || !file_path.is_file() {
            continue;
        }

        let file = File::open(&file_path).map_err(|e| {
            AppError::new(
                "Zip:Open",
                eyre!(format!(
                    "Path: {:?}, Error: {}",
                    file_entry.file_path.clone(),
                    e.to_string()
                )),
            )
        })?;
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        // Adding the file to the ZIP archive.
        if file_entry.sub_path.is_some() && file_entry.sub_path.clone().unwrap() != "" {
            let mut sub_path = file_entry.sub_path.clone().unwrap();
            if sub_path.starts_with("/") {
                sub_path = sub_path[1..].to_string();
            }
            if sub_path.ends_with("/") {
                sub_path = sub_path[..sub_path.len() - 1].to_string();
            }
            zip.start_file(format!("{}/{}", sub_path, file_name), options)
                .map_err(|e| {
                    AppError::new(
                        "Zip:StartSub",
                        eyre!(format!(
                            "Path: {:?}, ZipPath: {:?}, Error: {}",
                            file_entry.file_path.clone(),
                            file_entry.sub_path.clone(),
                            e.to_string()
                        )),
                    )
                })?;
        } else {
            zip.start_file(file_name, options).map_err(|e| {
                AppError::new(
                    "Zip:Start",
                    eyre!(format!(
                        "Path: {:?}, Error: {}",
                        file_entry.file_path,
                        e.to_string()
                    )),
                )
            })?;
        }
        let mut buffer = Vec::new();
        io::copy(&mut file.take(u64::MAX), &mut buffer).map_err(|e| {
            AppError::new(
                "Zip:Copy",
                eyre!(format!(
                    "Path: {:?}, Error: {}",
                    file_entry.file_path,
                    e.to_string()
                )),
            )
        })?;

        zip.write_all(&buffer).map_err(|e| {
            AppError::new(
                "Zip:Write",
                eyre!(format!(
                    "Path: {:?}, Error: {}",
                    file_entry.file_path,
                    e.to_string()
                )),
            )
        })?;
    }
    zip.finish()
        .map_err(|e| AppError::new("Zip:Done", eyre!(format!("Error: {}", e.to_string()))))?;
    Ok(())
}