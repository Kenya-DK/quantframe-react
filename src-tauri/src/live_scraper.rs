use crate::structs::PriceHistoryDto;
use crate::structs::Response;
use crate::structs::Settings;
use std::error::Error;
extern crate csv;
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde_json::from_str;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;
use tauri::Window;

// Structs for the Warframe Market API

#[derive(Clone)]
pub struct LiveScraper {
    is_running: Arc<AtomicBool>,
    window: Window,
    token: String,
    settings: Option<Settings>,
    csv_path: String,
    csv_backop_path: String,
}

impl LiveScraper {
    pub fn new(window: Window, token: String, csv_path: String, csv_backop_path: String) -> Self {
        LiveScraper {
            is_running: Arc::new(AtomicBool::new(false)),
            window,
            token,
            csv_path,
            csv_backop_path,
            settings: None,
        }
    }

    pub fn start_loop(&mut self, token: String, settings: Settings) {
        println!("Start loop live scraper");
        self.token = token;
        self.settings = Some(settings);

        self.is_running.store(true, Ordering::SeqCst);
        let is_running = Arc::clone(&self.is_running);

        let scraper = self.clone();
        tauri::async_runtime::spawn(async move {
            // A loop that takes output from the async process and sends it
            // to the webview via a Tauri Event
            while is_running.load(Ordering::SeqCst) {
                println!("Loop live scraper is running...");
                if let Err(e) = scraper.run().await {
                    println!("Error: {}", e);
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
    }

    pub fn stop_loop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        // Return the current value of is_running
        self.is_running.load(Ordering::SeqCst)
    }

    // Http Request to Warframe Market API
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let records = self.get_csv()?;

        for record in &records {
            println!("{:?}", record);
        }

        // let url = "https://api.warframe.market/v1/items";
        // let response: Result<Response<ResponseWFMPayload>, String> =
        //     self.perform_request(url).await;
        // match response {
        //     Ok(response) => {
        //         let items = response.data.payload.items;
        //         let mut items_name: Vec<String> = Vec::new();
        //         for item in items {
        //             items_name.push(item.item_name);
        //         }
        //         let items_name = items_name.join(", ");
        //         let settings = Settings {
        //             data: items_name.to_string(),
        //         };
        //     }
        //     Err(e) => {
        //         println!("Error: {}", e);
        //     }
        // }
        Ok(())
    }
    pub async fn perform_request<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<Response<T>, String> {
        // Create an HTTPS connector and client
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        // Make the GET request
        let uri: Uri = url.parse::<Uri>().map_err(|e| e.to_string())?;
        let res = client.get(uri).await.map_err(|e| e.to_string())?;
        let body = hyper::body::to_bytes(res.into_body())
            .await
            .map_err::<String, _>(|e| e.to_string())?;
        let json_str = String::from_utf8_lossy(&body).to_string();

        // Deserialize the JSON response into the generic type T
        let data: T = from_str(&json_str).map_err(|e| e.to_string())?;

        Ok(Response { data })
    }
    pub fn get_csv(&self) -> Result<Vec<PriceHistoryDto>, Box<dyn Error>> {
        println!("Get csv");
        println!("{}", &self.csv_path);
        let mut reader = csv::Reader::from_path(&self.csv_path)?;
        let mut result = Vec::new();

        for record in reader.deserialize() {
            let record: PriceHistoryDto = record?;
            result.push(record);
        }

        Ok(result)
    }
}
