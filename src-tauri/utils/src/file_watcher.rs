use core::*;
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{Error, LoggerOptions, get_location, info, trace, warning};

pub trait LineHandler: Send {
    fn process_line(&mut self, entry: &LineEntry) -> Result<(bool, bool), Error>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LineEntry {
    pub line: String,
    pub prev_line: String,
    pub ignore_combined: bool,
    pub date: i64,
}
impl LineEntry {
    pub fn new(line: String, prev_line: String, ignore_combined: bool) -> Self {
        LineEntry {
            line,
            prev_line,
            ignore_combined,
            date: chrono::Utc::now().timestamp_millis(),
        }
    }
}

pub struct FileWatcher {
    path: Arc<Mutex<String>>,
    last_pos: Arc<Mutex<u64>>,
    prev_line: Arc<Mutex<Option<String>>>,
    cache: Arc<Mutex<Vec<LineEntry>>>, // store cached lines
    handlers: Arc<Mutex<Vec<Box<dyn LineHandler + Send>>>>,
}

impl FileWatcher {
    pub fn new(path: impl Into<String>) -> Self {
        let path = path.into();
        let last_pos = if Path::new(&path).exists() {
            std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0)
        } else {
            0
        };
        FileWatcher {
            path: Arc::new(Mutex::new(path)),
            last_pos: Arc::new(Mutex::new(last_pos)),
            prev_line: Arc::new(Mutex::new(None)),
            cache: Arc::new(Mutex::new(Vec::new())),
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn reset(&self) {
        let mut last_pos = self.last_pos.lock().unwrap();
        let mut prev_line = self.prev_line.lock().unwrap();
        let mut cache = self.cache.lock().unwrap();
        *last_pos = 0;
        *prev_line = None;
        cache.clear();
    }
    pub fn set_path(&self, new_path: impl Into<String>) {
        let new_path = new_path.into();
        let mut path = self.path.lock().unwrap();
        if *path == new_path {
            return;
        }
        if !Path::new(&new_path).exists() {
            warning(
                "FileWatcher",
                &format!("FileWatcher switched to non-existing file: {}", new_path),
                &LoggerOptions::default(),
            );
            return;
        }
        let mut last_pos = self.last_pos.lock().unwrap();
        let mut prev_line = self.prev_line.lock().unwrap();
        let mut cache = self.cache.lock().unwrap();
        let current_file_size = std::fs::metadata(&new_path).map(|m| m.len()).unwrap_or(0);
        *path = new_path;
        *last_pos = current_file_size;
        *prev_line = None; // reset previous line
        cache.clear(); // reset cache

        info(
            "FileWatcher",
            &format!("FileWatcher switched to file: {}", *path),
            &LoggerOptions::default(),
        );
    }
    pub fn add_handler(&self, handler: Box<dyn LineHandler + Send>) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.push(handler);
    }
    pub fn watch(&self) -> Result<(), Error> {
        loop {
            let path = { self.path.lock().unwrap().clone() };

            if Path::new(&path).exists() {
                let mut file = if let Ok(f) = File::open(&path) {
                    f
                } else {
                    warning(
                        "FileWatcher",
                        &format!("Failed to open file: {}, retrying... in 5 seconds", path),
                        &LoggerOptions::default(),
                    );
                    thread::sleep(Duration::from_secs(5));
                    continue;
                };
                let current_file_size = if let Ok(metadata) = file.metadata() {
                    metadata.len()
                } else {
                    warning(
                        "FileWatcher",
                        &format!(
                            "Failed to get metadata for file: {}, retrying... in 5 seconds",
                            path
                        ),
                        &LoggerOptions::default(),
                    );
                    thread::sleep(Duration::from_secs(5));
                    continue;
                };
                let mut pos = self.last_pos.lock().unwrap();

                if (*pos > current_file_size || current_file_size < *pos) && current_file_size != 0
                {
                    let mut prev_line = self.prev_line.lock().unwrap();
                    let mut cache = self.cache.lock().unwrap();
                    *pos = 0;
                    *prev_line = None;
                    cache.clear();
                    trace(
                        "FileWatcher",
                        &format!(
                            "File truncated or rotated. Resetting position for file: {} | Current Position: {} | Current File Size: {}",
                            path, *pos, current_file_size
                        ),
                        &LoggerOptions::default(),
                    );
                }

                file.seek(SeekFrom::Start(*pos))?;

                let reader = BufReader::new(file);
                let mut ignore_combined = false;

                // Try to read lines, handling UTF-8 errors gracefully
                let lines = match self.read_lines_lossy(reader) {
                    Ok(lines) => lines,
                    Err(e) => {
                        e.with_location(get_location!()).log("file_watcher.log");
                        Vec::new()
                    }
                };

                for line in lines {
                    let mut prev = self.prev_line.lock().unwrap();

                    let prev_line_str = prev.as_deref().unwrap_or("");

                    let entry =
                        LineEntry::new(line.clone(), prev_line_str.to_string(), ignore_combined);

                    let mut handlers = self.handlers.lock().unwrap();
                    for handler in handlers.iter_mut() {
                        match handler.process_line(&entry) {
                            Ok((break_loop, combined)) => {
                                if break_loop {
                                    println!("Processing loop broken by handler");
                                }
                                ignore_combined = combined;
                            }
                            Err(e) => {
                                println!("Error processing line in handler: {}", e);
                            }
                        }
                    }
                    // Add line to cache
                    let mut cache = self.cache.lock().unwrap();
                    cache.push(entry.clone());
                    *prev = Some(line);
                }
                if current_file_size != 0 {
                    *pos = current_file_size;
                }
            } else {
                warning(
                    "FileWatcher",
                    &format!("File not found: {}, retrying... in 5 seconds", path),
                    &LoggerOptions::default(),
                );
                // Sleep longer if file does not exist 5 seconds
                thread::sleep(Duration::from_secs(5));
            }

            thread::sleep(Duration::from_millis(1));
        }
    }
    pub fn get_cached_lines_between(&self, mut start: usize, mut end: usize) -> Vec<LineEntry> {
        if start >= 1 {
            start = 1;
        }
        if start > end {
            std::mem::swap(&mut start, &mut end);
        }
        let cache = self.cache.lock().unwrap();
        cache[start..end.min(cache.len())].to_vec()
    }
    pub fn get_all_cached_lines(&self) -> Vec<LineEntry> {
        let cache = self.cache.lock().unwrap();
        cache.clone()
    }
    /// Read lines from a BufReader, handling invalid UTF-8 gracefully
    fn read_lines_lossy(&self, mut reader: BufReader<File>) -> Result<Vec<String>, Error> {
        use std::io::Read;

        let mut lines = Vec::new();
        let mut buffer = Vec::new();

        // Read all remaining bytes
        match reader.read_to_end(&mut buffer) {
            Ok(_) => {}
            Err(e) => {
                warning(
                    "FileWatcher",
                    &format!("Error reading file contents: {}", e),
                    &LoggerOptions::default(),
                );
                return Err(Error::from(e));
            }
        }

        if buffer.is_empty() {
            return Ok(lines);
        }

        // Convert to string with lossy UTF-8 conversion
        // This will replace invalid UTF-8 sequences with � (replacement character)
        let content = String::from_utf8_lossy(&buffer);

        // Split into lines
        for line in content.lines() {
            // Skip lines that contain only replacement characters (likely corrupted)
            if !line.chars().all(|c| c == '�') || line.is_empty() {
                lines.push(line.to_string());
            } else {
                trace(
                    "FileWatcher",
                    &format!("Skipping corrupted line with invalid UTF-8: {}", line),
                    &LoggerOptions::default(),
                );
            }
        }

        Ok(lines)
    }

    /// Alternative method for reading lines in chunks (more memory efficient for large files)
    #[allow(dead_code)]
    fn read_lines_chunked(&self, mut reader: BufReader<File>) -> Result<Vec<String>, Error> {
        use std::io::Read;

        let mut lines = Vec::new();
        let mut buffer = [0u8; 8192]; // 8KB chunks
        let mut incomplete_line = Vec::new();

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(bytes_read) => {
                    let mut current_chunk = Vec::new();
                    current_chunk.extend_from_slice(&incomplete_line);
                    current_chunk.extend_from_slice(&buffer[..bytes_read]);

                    // Convert chunk to string with lossy conversion
                    let chunk_str = String::from_utf8_lossy(&current_chunk);

                    let mut chunk_lines: Vec<&str> = chunk_str.lines().collect();

                    // Check if the chunk ends with a complete line
                    let ends_with_newline = current_chunk.ends_with(&[b'\n'])
                        || current_chunk.ends_with(&[b'\r', b'\n']);

                    if !ends_with_newline && !chunk_lines.is_empty() {
                        // Last line is incomplete, save it for next chunk
                        let incomplete = chunk_lines.pop().unwrap_or("");
                        incomplete_line = incomplete.as_bytes().to_vec();
                    } else {
                        incomplete_line.clear();
                    }

                    // Add complete lines
                    for line in chunk_lines {
                        if !line.chars().all(|c| c == '�') {
                            lines.push(line.to_string());
                        }
                    }
                }
                Err(e) => {
                    warning(
                        "FileWatcher",
                        &format!("Error reading file chunk: {}", e),
                        &LoggerOptions::default(),
                    );
                    return Err(Error::from(e));
                }
            }
        }

        // Handle any remaining incomplete line
        if !incomplete_line.is_empty() {
            let final_line = String::from_utf8_lossy(&incomplete_line);
            if !final_line.chars().all(|c| c == '�') {
                lines.push(final_line.to_string());
            }
        }

        Ok(lines)
    }
}
