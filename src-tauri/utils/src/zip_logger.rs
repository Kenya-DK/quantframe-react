use crate::helper::remove_ansi_codes;
use crate::{Error, LoggerOptions, OperationSet, get_location, info};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use zip::write::{FileOptions, ZipWriter};

#[derive(Clone)]
pub struct ZipLogger {
    component: String,
    start_time: Instant,
    log_entries: Arc<Mutex<String>>,
    files: Arc<Mutex<Vec<(String, Vec<u8>)>>>,
    pub operations: OperationSet,
}

impl ZipLogger {
    pub fn new() -> Self {
        Self {
            component: "ZipLogger".to_string(),
            start_time: Instant::now(),
            files: Arc::new(Mutex::new(Vec::new())),
            log_entries: Arc::new(Mutex::new(String::new())),
            operations: OperationSet::new(),
        }
    }

    /// Add a log entry to the zip archive
    pub fn add_log(&self, message: impl Into<String>) {
        let message = message.into();
        let elapsed = self.start_time.elapsed().as_secs_f64();
        // only show 0.000 format for elapsed time
        let log_entry = format!("[{:.3}]: {}\n", elapsed, message);

        // Clean log entry (remove ANSI codes)
        let clean_entry = remove_ansi_codes(log_entry);

        // Append to the accumulated log entries
        let mut entries = self.log_entries.lock().unwrap();
        if let Some(file) = self.operations.get_value_after("DumpLog") {
            let op = LoggerOptions::default()
                .set_file(file)
                .set_console(false)
                .set_show_elapsed_time(false)
                .set_show_component(false)
                .set_show_level(false);
            info(&self.component, &message, &op);
        }
        entries.push_str(&clean_entry);
    }
    pub fn create_file(&self, archive_path: impl Into<String>, content: impl AsRef<[u8]>) {
        let mut files = self.files.lock().unwrap();
        files.push((archive_path.into(), content.as_ref().to_vec()));
    }

    pub fn create_file_from_path(
        &self,
        archive_path: impl Into<String>,
        file_path: impl AsRef<Path>,
    ) -> Result<(), Error> {
        let archive_path = archive_path.into();
        let file_path = file_path.as_ref();

        let content = std::fs::read(file_path).map_err(|e| {
            Error::from_io(
                &format!("{}:CreateFileFromPath", self.component),
                &file_path.to_path_buf(),
                "reading source file",
                e,
                get_location!(),
            )
        })?;

        let mut files = self.files.lock().unwrap();
        files.push((archive_path, content));

        Ok(())
    }

    /// Finalize and close the zip archive
    pub fn finalize(&self, archive_name: impl Into<String>) -> Result<(), Error> {
        let archive_name = archive_name.into();
        let component = format!("{}:Finalize", self.component);

        let folder_path = crate::options::get_folder();
        let file_path = folder_path.join(&archive_name);

        let file = File::create(&file_path).map_err(|e| {
            Error::from_io(
                &component,
                &file_path,
                "creating zip file",
                e,
                get_location!(),
            )
        })?;

        let mut writer = ZipWriter::new(BufWriter::new(file));

        // 1. write stored files
        let files = self.files.lock().unwrap();
        for (name, data) in files.iter() {
            writer
                .start_file(name, FileOptions::default())
                .map_err(|e| {
                    Error::from_zip(
                        &component,
                        &archive_name,
                        "starting file in zip archive",
                        e,
                        get_location!(),
                    )
                })?;

            writer.write_all(data).map_err(|e| {
                Error::from_io(
                    &component,
                    &PathBuf::from(name),
                    "writing file to zip archive",
                    e,
                    get_location!(),
                )
            })?;
        }

        // 2. write logs
        let logs = self.log_entries.lock().unwrap();
        if !logs.is_empty() {
            writer
                .start_file("log.txt", FileOptions::default())
                .map_err(|e| {
                    Error::from_zip(
                        &component,
                        &archive_name,
                        "starting log file",
                        e,
                        get_location!(),
                    )
                })?;

            writer.write_all(logs.as_bytes()).map_err(|e| {
                Error::from_io(
                    &component,
                    &PathBuf::from("log.txt"),
                    "writing logs",
                    e,
                    get_location!(),
                )
            })?;
        }

        drop(files);
        drop(logs);

        // 3. finish zip
        writer.finish().map_err(|e| {
            Error::from_zip(
                &component,
                &archive_name,
                "finalizing zip archive",
                e,
                get_location!(),
            )
        })?;

        Ok(())
    }
}
