use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tauri::Window;

pub struct LiveScraper {
    is_running: Arc<AtomicBool>,
    window: Window,
    handle: Option<JoinHandle<()>>,
}

impl LiveScraper {
    pub fn new(window: Window) -> Self {
        LiveScraper {
            is_running: Arc::new(AtomicBool::new(false)),
            window,
            handle: None,
        }
    }

    pub fn start_loop(&mut self) {
        self.is_running.store(true, Ordering::SeqCst);
        let is_running = Arc::clone(&self.is_running);

        let window = self.window.clone();
        self.handle = Some(thread::spawn(move || {
            while is_running.load(Ordering::SeqCst) {
                println!("Loop live scraper is running...");
                thread::sleep(Duration::from_secs(1));

                window
                    .emit("loopRunning", ())
                    .expect("failed to emit event");
            }
        }));
    }

    pub fn stop_loop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        // Return the current value of is_running
        self.is_running.load(Ordering::SeqCst)
    }
}
