use chrono::{format, Duration};
use directories::{BaseDirs, UserDirs};
use eyre::eyre;
use once_cell::sync::Lazy;
use polars::{
    lazy::dsl::col,
    prelude::{DataFrame, Expr, IntoLazy, SortOptions},
    series::Series,
};
use serde_json::{json, Value};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::{api::file, Window};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use crate::{
    error::AppError,
    logger::{self},
    structs::WarframeLanguage,
    PACKAGEINFO,
};

/// Returns the path to the local data directory.
///
/// The function uses the directories crate to find the path to the local data directory.
/// If the local data directory cannot be found, the function panics.
///
/// # Returns
///
/// A PathBuf containing the path to the local data directory.
///
/// # Panics
///
/// The function panics if the local data directory cannot be found.
pub fn get_app_local_path() -> PathBuf {
    if let Some(base_dirs) = BaseDirs::new() {
        // App path for csv file
        let local_path = Path::new(base_dirs.data_local_dir());
        local_path.to_path_buf()
    } else {
        panic!("Could not find app path");
    }
}

/// Returns the path to the application's data directory.
///
/// The function uses the directories crate to find the path to the cache directory, and appends the application's specific path.
/// If the application's data directory does not exist, it is created.
/// If the base directories cannot be found, the function panics.
///
/// # Returns
///
/// A PathBuf containing the path to the application's data directory.
///
/// # Panics
///
/// The function panics if the base directories cannot be found.
pub fn get_app_data_path() -> PathBuf {
    if let Some(base_dirs) = BaseDirs::new() {
        // App path for csv file
        let roaming_path = Path::new(base_dirs.cache_dir());
        let app_path = roaming_path.join("dev.kenya.quantframe");
        // Check if the app path exists, if not create it
        if !app_path.exists() {
            fs::create_dir_all(app_path.clone()).unwrap();
        }
        app_path
    } else {
        panic!("Could not find app path");
    }
}

/// Returns the path to the current user's desktop directory.
///
/// The function uses the directories crate to find the path to the desktop directory.
/// It returns a PathBuf containing the path.
///
/// # Returns
///
/// A PathBuf containing the path to the current user's desktop directory.
pub fn get_desktop_path() -> PathBuf {
    if let Some(base_dirs) = UserDirs::new() {
        let local_path = get_app_roaming_path(); // Ensure local_path lives long enough

        let desktop_path = match base_dirs.desktop_dir() {
            Some(desktop_path) => desktop_path,
            None => local_path.as_path(),
        };
        desktop_path.to_path_buf()
    } else {
        panic!("Could not find app path");
    }
}