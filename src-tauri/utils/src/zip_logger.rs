use crate::helper::remove_ansi_codes;
use crate::{Error, delete_log, get_location};
use chrono::Local;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use zip::write::{FileOptions, ZipWriter};

pub struct ZipLogger {
    component: String,
    start_time: Instant,
    writer: Arc<Mutex<ZipWriter<BufWriter<File>>>>,
    archive_name: String,
    log_entries: Arc<Mutex<String>>,
}

impl ZipLogger {
    /// Start a new zip archive for logging
    pub fn start(archive_name: impl Into<String>) -> Result<Self, Error> {
        let component = String::from("ZipLogger");
        let name = archive_name.into();
        let folder_path = crate::options::get_folder();
        let file_path = folder_path.join(&name);

        let file = File::create(&file_path).map_err(|e| {
            Error::from_io(
                &format!("{}:{}", component, "Start"),
                &file_path,
                "creating zip file",
                e,
                get_location!(),
            )
        })?;
        let writer = ZipWriter::new(BufWriter::new(file));

        Ok(ZipLogger {
            component,
            start_time: Instant::now(),
            writer: Arc::new(Mutex::new(writer)),
            archive_name: name,
            log_entries: Arc::new(Mutex::new(String::new())),
        })
    }

    fn component(&self, suffix: &str) -> String {
        format!("{}:{}", self.component, suffix)
    }

    /// Add a log entry to the zip archive
    pub fn add_log(&self, message: impl Into<String>) -> Result<(), Error> {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let log_entry = format!("[{}]: {}\n", elapsed, message.into());

        // Clean log entry (remove ANSI codes)
        let clean_entry = remove_ansi_codes(log_entry);

        // Append to the accumulated log entries
        let mut entries = self.log_entries.lock().unwrap();
        entries.push_str(&clean_entry);

        Ok(())
    }

    /// Add a complete log file to the zip archive
    pub fn add_log_file(
        &self,
        file_path: impl AsRef<Path>,
        archive_path: impl Into<String>,
    ) -> Result<(), Error> {
        let folder_path = crate::options::get_folder();
        let file_path = folder_path.join(file_path.as_ref());
        let content = std::fs::read_to_string(file_path.clone()).map_err(|e| {
            Error::from_io(
                &self.component("AddLogFile"),
                &file_path,
                "reading log file",
                e,
                get_location!(),
            )
        })?;
        let clean_content = remove_ansi_codes(content);

        let mut writer = self.writer.lock().unwrap();
        writer
            .start_file(archive_path.into(), FileOptions::default())
            .map_err(|e| {
                Error::from_zip(
                    &self.component("AddLogFile"),
                    &self.archive_name,
                    "starting file in zip archive",
                    e,
                    get_location!(),
                )
            })?;
        writer.write_all(clean_content.as_bytes()).map_err(|e| {
            Error::from_io(
                &self.component("AddLogFile"),
                &file_path,
                "writing log file to zip archive",
                e,
                get_location!(),
            )
        })?;

        Ok(())
    }

    pub fn add_error(&self, error: &Error) -> Result<(), Error> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let error_file_name = format!("error_{}.log", timestamp);
        error.log(&error_file_name);
        if let Ok(_) = self.add_log_file(&error_file_name, &error_file_name) {
            delete_log(&error_file_name)?;
        }
        Ok(())
    }

    /// Add raw text content to the zip archive
    pub fn add_text_file(
        &self,
        content: impl Into<String>,
        file_name: impl Into<String>,
    ) -> Result<(), Error> {
        let clean_content = remove_ansi_codes(content.into());
        let file_name: String = file_name.into();

        let mut writer = self.writer.lock().unwrap();
        writer
            .start_file(&file_name, FileOptions::default())
            .map_err(|e| {
                Error::from_zip(
                    &self.component("AddTextFile"),
                    &self.archive_name,
                    "starting text file in zip archive",
                    e,
                    get_location!(),
                )
            })?;
        writer.write_all(clean_content.as_bytes()).map_err(|e| {
            Error::from_io(
                &self.component("AddTextFile"),
                &PathBuf::from(&file_name),
                "writing text file to zip archive",
                e,
                get_location!(),
            )
        })?;

        Ok(())
    }

    /// Finalize and close the zip archive
    pub fn finalize(&self) -> Result<(), Error> {
        let component = self.component("Finalize");

        // Extract and clear accumulated log entries
        let mut entries = self.log_entries.lock().unwrap();
        if !entries.is_empty() {
            let mut writer = self.writer.lock().unwrap();
            writer
                .start_file("log.txt", FileOptions::default())
                .map_err(|e| {
                    Error::from_zip(
                        &component,
                        &self.archive_name,
                        "starting log file in zip archive",
                        e,
                        get_location!(),
                    )
                })?;
            writer.write_all(entries.as_bytes()).map_err(|e| {
                Error::from_io(
                    &component,
                    &PathBuf::from("log.txt"),
                    "writing accumulated log entries to zip archive",
                    e,
                    get_location!(),
                )
            })?;
            entries.clear(); // prevent double-finalization writes
        }
        drop(entries);

        // Finish the zip writer
        let mut writer = self.writer.lock().unwrap();
        writer.finish().map_err(|e| {
            Error::from_zip(
                &component,
                &self.archive_name,
                "finalizing zip archive",
                e,
                get_location!(),
            )
        })?;
        Ok(())
    }

    /// Get the archive name
    pub fn archive_name(&self) -> &str {
        &self.archive_name
    }
}
