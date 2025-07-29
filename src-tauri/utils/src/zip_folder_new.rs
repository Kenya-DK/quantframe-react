use crate::*;
use serde_json::Value;
use std::path::PathBuf;

/// Configuration options for ZIP folder operations
#[derive(Default, Debug, Clone)]
pub struct ZipOptions<'a> {
    /// Whether to include hidden files and folders (starting with '.')
    pub include_hidden: bool,
    /// Optional list of JSON property names to mask in .json files
    pub mask_properties: Option<&'a [&'a str]>,
    /// Optional list of file/folder patterns to exclude (supports wildcards)
    pub exclude_patterns: Option<&'a [&'a str]>,
}

impl<'a> ZipOptions<'a> {
    /// Create new ZipOptions with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether to include hidden files and folders
    pub fn include_hidden(mut self, include: bool) -> Self {
        self.include_hidden = include;
        self
    }

    /// Set JSON properties to mask in .json files
    pub fn mask_properties(mut self, properties: &'a [&'a str]) -> Self {
        self.mask_properties = Some(properties);
        self
    }

    /// Set file/folder patterns to exclude
    pub fn exclude_patterns(mut self, patterns: &'a [&'a str]) -> Self {
        self.exclude_patterns = Some(patterns);
        self
    }
}

impl Error {
    /// Create a ZIP archive from a folder with configurable options
    ///
    /// # Arguments
    /// * `source_folder` - Path to the folder to compress
    /// * `zip_path` - Path where the ZIP file will be created
    /// * `options` - ZIP configuration options
    ///
    /// # Returns
    /// Result indicating success or failure
    ///
    /// # Examples
    /// ```
    /// // Basic ZIP
    /// Error::zip_folder_with_options("./logs", "./backup.zip", ZipOptions::new())?;
    ///
    /// // ZIP with exclusions
    /// Error::zip_folder_with_options(
    ///     "./project",
    ///     "./backup.zip",
    ///     ZipOptions::new()
    ///         .exclude_patterns(&["*.log", "temp/", "node_modules/"])
    /// )?;
    ///
    /// // ZIP with masking and exclusions
    /// Error::zip_folder_with_options(
    ///     "./config",
    ///     "./secure_backup.zip",
    ///     ZipOptions::new()
    ///         .mask_properties(&["password", "api_key"])
    ///         .exclude_patterns(&["*.log", "temp/"])
    ///         .include_hidden(false)
    /// )?;
    /// ```
    pub fn zip_folder_with_options(
        source_folder: impl Into<PathBuf>,
        zip_path: impl Into<PathBuf>,
        options: ZipOptions,
    ) -> Result<(), Error> {
        use std::fs::File;
        use std::io::{Read, Write};
        use zip::{ZipWriter, write::FileOptions};

        let source = source_folder.into();
        let zip_file_path = zip_path.into();

        // Ensure source folder exists
        if !source.exists() {
            return Err(Error::new(
                "ZipFolder",
                format!("Source folder does not exist: {}", source.display()),
                "Error::zip_folder_with_options",
            ));
        }

        if !source.is_dir() {
            return Err(Error::new(
                "ZipFolder",
                format!("Source path is not a directory: {}", source.display()),
                "Error::zip_folder_with_options",
            ));
        }

        // Create parent directory for zip file if it doesn't exist
        if let Some(parent) = zip_file_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                Error::from_io(
                    "ZipFolder",
                    &zip_file_path,
                    "creating parent directory for zip file",
                    e,
                    "Error::zip_folder_with_options",
                )
            })?;
        }

        // Create the zip file
        let zip_file = File::create(&zip_file_path).map_err(|e| {
            Error::from_io(
                "ZipFolder",
                &zip_file_path,
                "creating zip file",
                e,
                "Error::zip_folder_with_options",
            )
        })?;

        let mut zip = ZipWriter::new(zip_file);
        let file_options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        // Helper function to check if a path should be excluded
        fn should_exclude(path: &PathBuf, name: &str, exclude_patterns: Option<&[&str]>) -> bool {
            if let Some(patterns) = exclude_patterns {
                for pattern in patterns {
                    // Check for exact folder match (with trailing slash)
                    if pattern.ends_with('/') {
                        let folder_name = &pattern[..pattern.len() - 1];
                        if name == folder_name {
                            return true;
                        }
                    }
                    // Check for wildcard patterns
                    else if pattern.contains('*') {
                        // Simple wildcard matching for file extensions
                        if pattern.starts_with("*.") {
                            let extension = &pattern[2..];
                            if let Some(file_ext) = path.extension() {
                                if file_ext.to_string_lossy().to_lowercase()
                                    == extension.to_lowercase()
                                {
                                    return true;
                                }
                            }
                        }
                        // Wildcard at the end
                        else if pattern.ends_with('*') {
                            let prefix = &pattern[..pattern.len() - 1];
                            if name.starts_with(prefix) {
                                return true;
                            }
                        }
                        // Wildcard at the beginning
                        else if pattern.starts_with('*') {
                            let suffix = &pattern[1..];
                            if name.ends_with(suffix) {
                                return true;
                            }
                        }
                    }
                    // Check for exact name match
                    else if name == *pattern {
                        return true;
                    }
                }
            }
            false
        }

        // Walk through the directory
        fn zip_dir_recursive(
            zip: &mut ZipWriter<File>,
            source_dir: &PathBuf,
            prefix: &str,
            options: &ZipOptions,
            file_options: FileOptions,
        ) -> Result<(), Error> {
            let entries = std::fs::read_dir(source_dir).map_err(|e| {
                Error::from_io(
                    "ZipFolder",
                    source_dir,
                    "reading directory entries",
                    e,
                    "zip_dir_recursive",
                )
            })?;

            for entry in entries {
                let entry = entry.map_err(|e| {
                    Error::from_io(
                        "ZipFolder",
                        source_dir,
                        "processing directory entry",
                        e,
                        "zip_dir_recursive",
                    )
                })?;

                let path = entry.path();
                let name = entry.file_name();
                let name_str = name.to_string_lossy();

                // Skip hidden files/folders if not included
                if !options.include_hidden && name_str.starts_with('.') {
                    continue;
                }

                // Check if this file/folder should be excluded
                if should_exclude(&path, &name_str, options.exclude_patterns) {
                    continue;
                }

                let zip_path = if prefix.is_empty() {
                    name_str.to_string()
                } else {
                    format!("{}/{}", prefix, name_str)
                };

                if path.is_file() {
                    // Add file to zip
                    zip.start_file(&zip_path, file_options).map_err(|e| {
                        Error::from_zip(
                            "ZipFolder",
                            &zip_path,
                            "starting file in zip",
                            e,
                            "zip_dir_recursive",
                        )
                    })?;

                    // Check if this is a JSON file and masking is enabled
                    if let Some(mask_props) = options.mask_properties {
                        if let Some(extension) = path.extension() {
                            if extension.to_string_lossy().to_lowercase() == "json" {
                                // Read and mask JSON file
                                let content = std::fs::read_to_string(&path).map_err(|e| {
                                    Error::from_io(
                                        "ZipFolder",
                                        &path,
                                        "reading JSON file for masking",
                                        e,
                                        "zip_dir_recursive",
                                    )
                                })?;

                                match serde_json::from_str::<Value>(&content) {
                                    Ok(mut json_value) => {
                                        // Mask sensitive data if it's a JSON object
                                        if let Value::Object(ref mut obj) = json_value {
                                            crate::helper::mask_sensitive_data(obj, mask_props);
                                        }

                                        // Serialize the masked JSON
                                        let masked_content = serde_json::to_string_pretty(
                                            &json_value,
                                        )
                                        .map_err(|e| {
                                            Error::from_json(
                                                "ZipFolder",
                                                &path,
                                                "masked_json",
                                                "Failed to serialize masked JSON",
                                                e,
                                                "zip_dir_recursive",
                                            )
                                        })?;

                                        // Write masked content to zip
                                        zip.write_all(masked_content.as_bytes()).map_err(|e| {
                                            Error::from_zip(
                                                "ZipFolder",
                                                &zip_path,
                                                "writing masked JSON to zip",
                                                zip::result::ZipError::Io(e),
                                                "zip_dir_recursive",
                                            )
                                        })?;
                                    }
                                    Err(_) => {
                                        // If JSON parsing fails, include the original file
                                        let mut file = File::open(&path).map_err(|e| {
                                            Error::from_io(
                                                "ZipFolder",
                                                &path,
                                                "opening invalid JSON file for zip",
                                                e,
                                                "zip_dir_recursive",
                                            )
                                        })?;

                                        let mut buffer = Vec::new();
                                        file.read_to_end(&mut buffer).map_err(|e| {
                                            Error::from_io(
                                                "ZipFolder",
                                                &path,
                                                "reading invalid JSON file for zip",
                                                e,
                                                "zip_dir_recursive",
                                            )
                                        })?;

                                        zip.write_all(&buffer).map_err(|e| {
                                            Error::from_zip(
                                                "ZipFolder",
                                                &zip_path,
                                                "writing invalid JSON file to zip",
                                                zip::result::ZipError::Io(e),
                                                "zip_dir_recursive",
                                            )
                                        })?;
                                    }
                                }
                            } else {
                                // Non-JSON file, include as-is
                                let mut file = File::open(&path).map_err(|e| {
                                    Error::from_io(
                                        "ZipFolder",
                                        &path,
                                        "opening file for zip",
                                        e,
                                        "zip_dir_recursive",
                                    )
                                })?;

                                let mut buffer = Vec::new();
                                file.read_to_end(&mut buffer).map_err(|e| {
                                    Error::from_io(
                                        "ZipFolder",
                                        &path,
                                        "reading file for zip",
                                        e,
                                        "zip_dir_recursive",
                                    )
                                })?;

                                zip.write_all(&buffer).map_err(|e| {
                                    Error::from_zip(
                                        "ZipFolder",
                                        &zip_path,
                                        "writing file to zip",
                                        zip::result::ZipError::Io(e),
                                        "zip_dir_recursive",
                                    )
                                })?;
                            }
                        } else {
                            // File without extension, include as-is
                            let mut file = File::open(&path).map_err(|e| {
                                Error::from_io(
                                    "ZipFolder",
                                    &path,
                                    "opening file for zip",
                                    e,
                                    "zip_dir_recursive",
                                )
                            })?;

                            let mut buffer = Vec::new();
                            file.read_to_end(&mut buffer).map_err(|e| {
                                Error::from_io(
                                    "ZipFolder",
                                    &path,
                                    "reading file for zip",
                                    e,
                                    "zip_dir_recursive",
                                )
                            })?;

                            zip.write_all(&buffer).map_err(|e| {
                                Error::from_zip(
                                    "ZipFolder",
                                    &zip_path,
                                    "writing file to zip",
                                    zip::result::ZipError::Io(e),
                                    "zip_dir_recursive",
                                )
                            })?;
                        }
                    } else {
                        // No masking, include file as-is
                        let mut file = File::open(&path).map_err(|e| {
                            Error::from_io(
                                "ZipFolder",
                                &path,
                                "opening file for zip",
                                e,
                                "zip_dir_recursive",
                            )
                        })?;

                        let mut buffer = Vec::new();
                        file.read_to_end(&mut buffer).map_err(|e| {
                            Error::from_io(
                                "ZipFolder",
                                &path,
                                "reading file for zip",
                                e,
                                "zip_dir_recursive",
                            )
                        })?;

                        zip.write_all(&buffer).map_err(|e| {
                            Error::from_zip(
                                "ZipFolder",
                                &zip_path,
                                "writing file to zip",
                                zip::result::ZipError::Io(e),
                                "zip_dir_recursive",
                            )
                        })?;
                    }
                } else if path.is_dir() {
                    // Add directory (create empty directory entry)
                    zip.add_directory(&format!("{}/", zip_path), file_options)
                        .map_err(|e| {
                            Error::from_zip(
                                "ZipFolder",
                                &zip_path,
                                "adding directory to zip",
                                e,
                                "zip_dir_recursive",
                            )
                        })?;

                    // Recursively add directory contents
                    zip_dir_recursive(zip, &path, &zip_path, options, file_options)?;
                }
            }
            Ok(())
        }

        // Start the recursive zipping
        zip_dir_recursive(&mut zip, &source, "", &options, file_options)?;

        // Finish the zip file
        zip.finish().map_err(|e| {
            Error::from_zip(
                "ZipFolder",
                zip_file_path.to_string_lossy(),
                "finalizing zip file",
                e,
                "Error::zip_folder_with_options",
            )
        })?;

        Ok(())
    }

    /// Create a ZIP archive from a folder (convenience method for backward compatibility)
    ///
    /// # Arguments
    /// * `source_folder` - Path to the folder to compress
    /// * `zip_path` - Path where the ZIP file will be created
    /// * `include_hidden` - Whether to include hidden files and folders
    ///
    /// # Returns
    /// Result indicating success or failure
    ///
    /// # Example
    /// ```
    /// Error::zip_folder("./logs", "./backup/logs.zip", false)?;
    /// ```
    pub fn zip_folder(
        source_folder: impl Into<PathBuf>,
        zip_path: impl Into<PathBuf>,
        include_hidden: bool,
    ) -> Result<(), Error> {
        Self::zip_folder_with_options(
            source_folder,
            zip_path,
            ZipOptions::new().include_hidden(include_hidden),
        )
    }

    /// Create a ZIP archive from a folder with exclusions (convenience method)
    ///
    /// # Arguments
    /// * `source_folder` - Path to the folder to compress
    /// * `zip_path` - Path where the ZIP file will be created
    /// * `include_hidden` - Whether to include hidden files and folders
    /// * `exclude_patterns` - List of file/folder patterns to exclude
    ///
    /// # Returns
    /// Result indicating success or failure
    ///
    /// # Example
    /// ```
    /// Error::zip_folder_with_exclusions(
    ///     "./project",
    ///     "./backup/project.zip",
    ///     false,
    ///     &["*.log", "temp/", "node_modules/"]
    /// )?;
    /// ```
    pub fn zip_folder_with_exclusions(
        source_folder: impl Into<PathBuf>,
        zip_path: impl Into<PathBuf>,
        include_hidden: bool,
        exclude_patterns: &[&str],
    ) -> Result<(), Error> {
        Self::zip_folder_with_options(
            source_folder,
            zip_path,
            ZipOptions::new()
                .include_hidden(include_hidden)
                .exclude_patterns(exclude_patterns),
        )
    }

    /// Create a ZIP archive from a folder with masking (convenience method)
    ///
    /// # Arguments
    /// * `source_folder` - Path to the folder to compress
    /// * `zip_path` - Path where the ZIP file will be created
    /// * `include_hidden` - Whether to include hidden files and folders
    /// * `mask_properties` - JSON properties to mask
    /// * `exclude_patterns` - Optional list of file/folder patterns to exclude
    ///
    /// # Returns
    /// Result indicating success or failure
    ///
    /// # Example
    /// ```
    /// Error::zip_folder_with_masking(
    ///     "./config",
    ///     "./backup/config.zip",
    ///     false,
    ///     Some(&["password", "api_key"]),
    ///     Some(&["*.log", "temp/"])
    /// )?;
    /// ```
    pub fn zip_folder_with_masking(
        source_folder: impl Into<PathBuf>,
        zip_path: impl Into<PathBuf>,
        include_hidden: bool,
        mask_properties: Option<&[&str]>,
        exclude_patterns: Option<&[&str]>,
    ) -> Result<(), Error> {
        let mut options = ZipOptions::new().include_hidden(include_hidden);

        if let Some(mask_props) = mask_properties {
            options = options.mask_properties(mask_props);
        }

        if let Some(exclude_props) = exclude_patterns {
            options = options.exclude_patterns(exclude_props);
        }

        Self::zip_folder_with_options(source_folder, zip_path, options)
    }
}
