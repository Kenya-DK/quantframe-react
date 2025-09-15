use core::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::{Error, LoggerOptions, info, trace, warning};

pub trait LineHandler: Send {
    fn process_line(
        &mut self,
        line: &str,
        prev_line: &str,
        ignore_combined: bool,
    ) -> Result<(bool, bool), Error>;
}

pub struct FileWatcher {
    path: Arc<Mutex<String>>,
    last_pos: Arc<Mutex<u64>>,
    prev_line: Arc<Mutex<Option<String>>>,
    cache: Arc<Mutex<Vec<String>>>, // store cached lines
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
                let mut file = File::open(&path)?;
                let current_file_size = std::fs::metadata(&path)?.len();
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
                for line in reader.lines() {
                    let line = line?;
                    let mut prev = self.prev_line.lock().unwrap();

                    let prev_line_str = prev.as_deref().unwrap_or("");

                    let mut handlers = self.handlers.lock().unwrap();
                    for handler in handlers.iter_mut() {
                        match handler.process_line(&line, prev_line_str, ignore_combined) {
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
                    // let mut cache = self.cache.lock().unwrap();
                    // cache.push(line.clone());
                    *prev = Some(line);
                }
                if current_file_size != 0 {
                    *pos = current_file_size;
                }
            } else {
                warning(
                    "FileWatcher",
                    &format!("File not found: {}", path),
                    &LoggerOptions::default(),
                );
            }

            thread::sleep(Duration::from_millis(1));
        }
    }
    pub fn get_cached_lines_between(&self, mut start: usize, mut end: usize) -> Vec<String> {
        if start >= 1 {
            start = 1;
        }
        if start > end {
            std::mem::swap(&mut start, &mut end);
        }
        let cache = self.cache.lock().unwrap();
        cache[start..end.min(cache.len())].to_vec()
    }
}
