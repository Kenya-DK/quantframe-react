use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, OnceLock},
    thread::{self, JoinHandle},
};
use utils::*;

// ---------- Models ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    #[serde(default)]
    pub item: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemQueryParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "volumeGt")]
    pub volume_gt: Option<f64>,
    #[serde(rename = "volumeLt")]
    pub volume_lt: Option<f64>,
    #[serde(rename = "supplyGt")]
    pub supply_gt: Option<f64>,
    #[serde(rename = "supplyLt")]
    pub supply_lt: Option<f64>,
    #[serde(rename = "demandGt")]
    pub demand_gt: Option<f64>,
    #[serde(rename = "demandLt")]
    pub demand_lt: Option<f64>,
    #[serde(rename = "minPriceGt")]
    pub min_price_gt: Option<f64>,
    #[serde(rename = "minPriceLt")]
    pub min_price_lt: Option<f64>,
    #[serde(rename = "maxPriceGt")]
    pub max_price_gt: Option<f64>,
    #[serde(rename = "maxPriceLt")]
    pub max_price_lt: Option<f64>,
}

// ---------- URL Parser ----------
fn parse_query_params(query_string: &str) -> HashMap<String, Vec<String>> {
    let mut params = HashMap::new();

    if query_string.is_empty() {
        return params;
    }

    for pair in query_string.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            // Simple URL decode (just replace %20 with space for basic cases)
            let decoded_key = key.replace("%20", " ");
            let decoded_value = value.replace("%20", " ");

            params
                .entry(decoded_key)
                .or_insert_with(Vec::new)
                .push(decoded_value);
        }
    }

    params
}

fn convert_url_to_json(url: &str) -> Value {
    // Parse the URL to extract query parameters
    if let Some(query_start) = url.find('?') {
        let (base_url, query_string) = url.split_at(query_start + 1);
        let base_path = &base_url[..base_url.len() - 1]; // Remove the '?'

        let params = parse_query_params(query_string);
        let mut query_params = ItemQueryParams {
            page: None,
            limit: None,
            sort_by: None,
            sort_direction: None,
            from_date: None,
            to_date: None,
            tags: None,
            volume_gt: None,
            volume_lt: None,
            supply_gt: None,
            supply_lt: None,
            demand_gt: None,
            demand_lt: None,
            min_price_gt: None,
            min_price_lt: None,
            max_price_gt: None,
            max_price_lt: None,
        };

        // Populate the struct from parsed parameters
        for (key, values) in params {
            match key.as_str() {
                "page" => query_params.page = values.first().and_then(|v| v.parse().ok()),
                "limit" => query_params.limit = values.first().and_then(|v| v.parse().ok()),
                "sort_by" => query_params.sort_by = values.first().cloned(),
                "sort_direction" => query_params.sort_direction = values.first().cloned(),
                "from_date" => query_params.from_date = values.first().cloned(),
                "to_date" => query_params.to_date = values.first().cloned(),
                "tags" => query_params.tags = Some(values),
                "volumeGt" => query_params.volume_gt = values.first().and_then(|v| v.parse().ok()),
                "volumeLt" => query_params.volume_lt = values.first().and_then(|v| v.parse().ok()),
                "supplyGt" => query_params.supply_gt = values.first().and_then(|v| v.parse().ok()),
                "supplyLt" => query_params.supply_lt = values.first().and_then(|v| v.parse().ok()),
                "demandGt" => query_params.demand_gt = values.first().and_then(|v| v.parse().ok()),
                "demandLt" => query_params.demand_lt = values.first().and_then(|v| v.parse().ok()),
                "minPriceGt" => {
                    query_params.min_price_gt = values.first().and_then(|v| v.parse().ok())
                }
                "minPriceLt" => {
                    query_params.min_price_lt = values.first().and_then(|v| v.parse().ok())
                }
                "maxPriceGt" => {
                    query_params.max_price_gt = values.first().and_then(|v| v.parse().ok())
                }
                "maxPriceLt" => {
                    query_params.max_price_lt = values.first().and_then(|v| v.parse().ok())
                }
                _ => {} // Ignore unknown parameters
            }
        }

        json!({
            "url": base_path,
            "query_parameters": query_params
        })
    } else {
        json!({
            "url": url,
            "query_parameters": null
        })
    }
}

// ---------- Client ----------
#[derive(Debug)]
pub struct Client {
    self_arc: OnceLock<Arc<Client>>,
    order_route: OnceLock<Arc<OrderRoute>>,
    server_thread: Mutex<Option<JoinHandle<()>>>,
    running: Arc<Mutex<bool>>,
    host: Mutex<String>,
}

impl Client {
    pub fn new() -> Arc<Self> {
        let client = Arc::new(Self {
            self_arc: OnceLock::new(),
            order_route: OnceLock::new(),
            server_thread: Mutex::new(None),
            running: Arc::new(Mutex::new(false)),
            host: Mutex::new("127.0.0.1:8080".to_string()),
        });
        client.self_arc.set(client.clone()).unwrap();
        client
    }

    pub fn order(&self) -> Arc<OrderRoute> {
        self.order_route.get_or_init(|| OrderRoute::new()).clone()
    }

    pub fn set_host(&self, new_host: &str) {
        let mut host = self.host.lock().unwrap();
        *host = new_host.to_string();
    }

    pub fn start(&self) {
        let mut running = self.running.lock().unwrap();
        if *running {
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
                for stream in listener.incoming() {
                    let running = *running_flag.lock().unwrap();
                    if !running {
                        println!("🛑 Server stopped listening.");
                        break;
                    }

                    if let Ok(stream) = stream {
                        let client_clone = client.clone();
                        thread::spawn(move || handle_client(stream, client_clone));
                    }
                }
            } else {
                println!("❌ Failed to bind to {}", host);
            }
        });

        *self.server_thread.lock().unwrap() = Some(handle);
    }

    pub fn stop(&self) {
        let mut running = self.running.lock().unwrap();
        if !*running {
            println!("⚠️ Server is not running.");
            return;
        }

        *running = false;
        println!("🛑 Stopping server...");

        // Drop the thread handle safely
        if let Some(handle) = self.server_thread.lock().unwrap().take() {
            let _ = handle.join();
        }
        println!("✅ Server stopped.");
    }
}

// ---------- Order Route ----------
#[derive(Debug)]
pub struct OrderRoute {
    orders: Mutex<Vec<Order>>,
}

impl OrderRoute {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            orders: Mutex::new(Vec::new()),
        })
    }

    fn create_order(&self, id: &str, item: Option<&str>) {
        let mut orders = self.orders.lock().unwrap();
        let order = Order {
            id: id.to_string(),
            item: item.unwrap_or_default().to_string(),
        };
        orders.push(order);
    }

    fn list_orders(&self) -> Vec<Order> {
        self.orders.lock().unwrap().clone()
    }

    fn get_order(&self, id: &str) -> Option<Order> {
        self.orders
            .lock()
            .unwrap()
            .iter()
            .cloned()
            .find(|o| o.id == id)
    }

    fn delete_order(&self, id: &str) -> bool {
        let mut orders = self.orders.lock().unwrap();
        let before = orders.len();
        orders.retain(|o| o.id != id);
        before != orders.len()
    }

    fn patch_order(&self, id: &str, data: &Order) -> Option<Order> {
        let mut orders = self.orders.lock().unwrap();
        if let Some(order) = orders.iter_mut().find(|o| o.id == id) {
            if !data.item.is_empty() {
                order.item = data.item.clone();
            }
            return Some(order.clone());
        }
        None
    }

    pub fn handle_request(&self, method: &str, path: &str, body: &str, stream: &mut TcpStream) {
        if method == "POST" && path == "/stock_riven" {
            self.handle_post(body, stream);
        } else if method == "GET" && path == "/stock_riven" {
            self.handle_get_all(stream);
        } else if method == "GET" && path.starts_with("/stock_riven/") {
            self.handle_get_by_id(path, stream);
        } else if method == "DELETE" && path.starts_with("/stock_riven/") {
            self.handle_delete(path, stream);
        } else if method == "PATCH" && path.starts_with("/stock_riven/") {
            self.handle_patch(path, body, stream);
        } else if method == "POST" && path == "/parse-url" {
            self.handle_parse_url(body, stream);
        } else if method == "OPTIONS" {
            respond_cors_preflight(stream);
        } else {
            let _ = stream.write_all(
                b"HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nAccess-Control-Allow-Origin: *\r\n\r\nEndpoint not found",
            );
        }
    }

    fn handle_post(&self, body: &str, stream: &mut TcpStream) {
        match serde_json::from_str::<Order>(body) {
            Ok(order_data) => {
                self.create_order(&order_data.id, Some(&order_data.item));
                let response_body = serde_json::to_string(&self.list_orders()).unwrap();
                respond_json(stream, 200, &response_body);
            }
            Err(e) => respond_text(stream, 400, &format!("Invalid JSON: {}", e)),
        }
    }

    fn handle_get_all(&self, stream: &mut TcpStream) {
        let body = serde_json::to_string(&self.list_orders()).unwrap();
        respond_json(stream, 200, &body);
    }

    fn handle_get_by_id(&self, path: &str, stream: &mut TcpStream) {
        let id = path.trim_start_matches("/order/");
        match self.get_order(id) {
            Some(order) => respond_json(stream, 200, &serde_json::to_string(&order).unwrap()),
            None => respond_text(stream, 404, "Order not found"),
        }
    }

    fn handle_delete(&self, path: &str, stream: &mut TcpStream) {
        let id = path.trim_start_matches("/order/");
        if self.delete_order(id) {
            respond_text(stream, 200, "Order deleted");
        } else {
            respond_text(stream, 404, "Order not found");
        }
    }

    fn handle_patch(&self, path: &str, body: &str, stream: &mut TcpStream) {
        let id = path.trim_start_matches("/order/");
        match serde_json::from_str::<Order>(body) {
            Ok(data) => match self.patch_order(id, &data) {
                Some(order) => respond_json(stream, 200, &serde_json::to_string(&order).unwrap()),
                None => respond_text(stream, 404, "Order not found"),
            },
            Err(_) => respond_text(stream, 400, "Invalid JSON"),
        }
    }

    fn handle_parse_url(&self, body: &str, stream: &mut TcpStream) {
        #[derive(Deserialize)]
        struct UrlRequest {
            url: String,
        }

        match serde_json::from_str::<UrlRequest>(body) {
            Ok(request) => {
                let parsed_json = convert_url_to_json(&request.url);
                let response_body = serde_json::to_string_pretty(&parsed_json).unwrap();
                respond_json(stream, 200, &response_body);
            }
            Err(e) => respond_text(stream, 400, &format!("Invalid JSON: {}", e)),
        }
    }
}

// ---------- Helpers ----------
fn respond_json(stream: &mut TcpStream, status: u16, body: &str) {
    let response = format!(
        "HTTP/1.1 {} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE, PATCH, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type, Authorization\r\n\r\n{}",
        status,
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());
}

fn respond_text(stream: &mut TcpStream, status: u16, msg: &str) {
    let response = format!(
        "HTTP/1.1 {} OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE, PATCH, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type, Authorization\r\n\r\n{}",
        status,
        msg.len(),
        msg
    );
    let _ = stream.write_all(response.as_bytes());
}

fn respond_cors_preflight(stream: &mut TcpStream) {
    let response = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE, PATCH, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type, Authorization\r\nAccess-Control-Max-Age: 86400\r\nContent-Length: 0\r\n\r\n";
    let _ = stream.write_all(response.as_bytes());
}

// ---------- HTTP SERVER ----------
fn handle_client(mut stream: TcpStream, client: Arc<Client>) {
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

        let order_route = client.order();
        order_route.handle_request(method, path, body, &mut stream);
    }
}

// ---------- MAIN ----------
fn main() {
    let client = Client::new();

    client.set_host("127.0.0.1:9090");

    println!("🚀 Starting HTTP server...");
    println!("📡 You can now send POST requests to http://127.0.0.1:9090/parse-url");
    println!("📝 Request body format: {{\"url\": \"your_url_here\"}}");
    println!();

    client.start();

    // Keep the server running
    // client.stop();
    std::thread::sleep(std::time::Duration::from_secs(2222222222));
}
