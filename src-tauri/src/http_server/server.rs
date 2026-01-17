use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, OnceLock},
    thread::{self, JoinHandle},
    time::Duration,
};
use utils::*;

use crate::http_server::{respond_cors_preflight, StockItemRoute, StockRivenRoute};

#[derive(Debug)]
pub struct HttpServer {
    self_arc: OnceLock<Arc<HttpServer>>,
    stock_item_route: OnceLock<Arc<StockItemRoute>>,
    stock_riven_route: OnceLock<Arc<StockRivenRoute>>,
    server_thread: Mutex<Option<JoinHandle<()>>>,
    running: Arc<Mutex<bool>>,
    host: Mutex<String>,
}

impl HttpServer {
    pub fn new(host: &str, port: u16) -> Arc<Self> {
        let client = Arc::new(Self {
            self_arc: OnceLock::new(),
            stock_item_route: OnceLock::new(),
            stock_riven_route: OnceLock::new(),
            server_thread: Mutex::new(None),
            running: Arc::new(Mutex::new(false)),
            host: Mutex::new(format!("{}:{}", host, port)),
        });
        client.self_arc.set(client.clone()).unwrap();
        client
    }

    pub fn stock_item(&self) -> Arc<StockItemRoute> {
        self.stock_item_route
            .get_or_init(|| StockItemRoute::new())
            .clone()
    }
    pub fn stock_riven(&self) -> Arc<StockRivenRoute> {
        self.stock_riven_route
            .get_or_init(|| StockRivenRoute::new())
            .clone()
    }
    pub fn set_host(&self, new_host: impl Into<String>, port: u16) -> String {
        let new_host = format!("{}:{}", new_host.into(), port);
        let mut host = self.host.lock().unwrap();
        if *host != new_host {
            *host = new_host.to_string();
            String::from("CHANGED")
        } else {
            String::from("NO_CHANGE")
        }
    }
    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }
    pub fn start(&self) {
        let mut running = self.running.lock().unwrap();
        if *running {
            warning(
                "HTTPServer",
                "⚠️ Server is already running.",
                &LoggerOptions::default(),
            );
            println!("⚠️ Server is already running.");
            return;
        }

        let host = self.host.lock().unwrap().clone();
        info(
            "HTTPServer",
            &format!("🚀 Starting server on http://{}", host),
            &LoggerOptions::default(),
        );
        *running = true;
        drop(running);

        let client = self.self_arc.get().unwrap().clone();
        let running_flag = self.running.clone();

        let handle = thread::spawn(move || {
            if let Ok(listener) = TcpListener::bind(&host) {
                // Set non-blocking mode to allow periodic checks
                if let Err(e) = listener.set_nonblocking(true) {
                    error(
                        "HTTPServer",
                        &format!("❌ Failed to set non-blocking mode: {}", e),
                        &LoggerOptions::default(),
                    );
                    return;
                }

                loop {
                    // Check if we should stop
                    let running = *running_flag.lock().unwrap();
                    if !running {
                        break;
                    }

                    // Try to accept a connection (non-blocking)
                    match listener.accept() {
                        Ok((stream, _)) => {
                            let client_clone = client.clone();
                            tauri::async_runtime::spawn(async move {
                                handle_client(stream, client_clone).await
                            });
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            // No connection available, sleep briefly and continue
                            thread::sleep(Duration::from_millis(50));
                        }
                        Err(e) => {
                            error(
                                "HTTPServer",
                                &format!("❌ Error accepting connection: {}", e),
                                &LoggerOptions::default(),
                            );
                            // Brief pause before retrying
                            thread::sleep(Duration::from_millis(50));
                        }
                    }
                }
            } else {
                error(
                    "HTTPServer",
                    &format!("❌ Failed to bind to {}", host),
                    &LoggerOptions::default(),
                );
            }
        });

        *self.server_thread.lock().unwrap() = Some(handle);
    }
    pub fn restart(&self) {
        if self.is_running() {
            self.stop();
        }
        self.start();
    }
    pub fn stop(&self) {
        let mut running = self.running.lock().unwrap();
        if !*running {
            warning(
                "HTTPServer",
                "⚠️ Server is not running.",
                &LoggerOptions::default(),
            );
            return;
        }

        *running = false;
        drop(running); // Release lock before waiting for thread

        info(
            "HTTPServer",
            "🛑 Stopping server...",
            &LoggerOptions::default(),
        );

        // Wait for thread to finish gracefully
        if let Some(handle) = self.server_thread.lock().unwrap().take() {
            // Since we're using non-blocking I/O with periodic checks,
            // the thread should exit within a reasonable time (max ~100ms)
            match handle.join() {
                Ok(_) => {}
                Err(_) => {
                    warning(
                        "HTTPServer",
                        "⚠️ Server thread panicked during shutdown.",
                        &LoggerOptions::default(),
                    );
                }
            }
        }

        info(
            "HTTPServer",
            "✅ Server stopped.",
            &LoggerOptions::default(),
        );
    }

    /// Force stop the server (use only if regular stop() hangs)
    #[allow(dead_code)]
    pub fn force_stop(&self) {
        let mut running = self.running.lock().unwrap();
        *running = false;
        drop(running);

        warning(
            "HTTPServer",
            "⚠️ Force stopping server (not waiting for thread to join).",
            &LoggerOptions::default(),
        );

        // Just drop the thread handle without joining
        *self.server_thread.lock().unwrap() = None;

        info(
            "HTTPServer",
            "✅ Server force stopped.",
            &LoggerOptions::default(),
        );
    }
}

// ---------- HTTP SERVER ----------
async fn handle_client(mut stream: TcpStream, client: Arc<HttpServer>) {
    let mut buffer = [0; 4096];
    if let Ok(size) = stream.read(&mut buffer) {
        if size == 0 {
            return;
        }

        let request = String::from_utf8_lossy(&buffer[..size]);
        let request_line = request.lines().next().unwrap_or("");
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 2 {
            return;
        }

        let method = parts[0];
        let path = parts[1];
        let body = request.split("\r\n\r\n").nth(1).unwrap_or("");

        // Extract Origin header for CORS handling
        if method == "OPTIONS" {
            respond_cors_preflight(&mut stream);
            return;
        }

        let stock_item_route = client.stock_item();
        stock_item_route
            .handle_request(method, path, body, &mut stream)
            .await;

        let stock_riven_route = client.stock_riven();
        stock_riven_route
            .handle_request(method, path, body, &mut stream)
            .await;
    }
}
