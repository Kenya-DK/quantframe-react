use std::sync::{Arc, Mutex};

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use crate::{settings::SettingsState, utils::modules::{error::AppError, logger}};

use super::modules::{conversation::new_conversation, trade::progress, stock::add_riven};
#[derive(Clone, Debug)]
pub struct HttpClient {}

impl HttpClient {
    pub fn setup(settings: Arc<Mutex<SettingsState>>) -> Result<Self, AppError> {
        let settings = settings.lock().unwrap();
        tauri::async_runtime::spawn(
            HttpServer::new(|| {
                App::new()
                    .wrap(
                        Cors::default()
                            .allow_any_origin()
                            .allowed_headers(vec!["Authorization", "Content-Type", "User-Agent"])
                            .allow_any_method()
                            .expose_any_header(),
                    )
                    .service(web::scope("/stock")
                        .service(add_riven)
                        // .service(add_item)
                    )
                    .service(web::scope("/trading").service(progress))
                    .service(new_conversation)
            })
            .bind((settings.http.clone().host, settings.http.port as u16))
            .map_err(|e| AppError::new("HttpServer", eyre::eyre!(e)))?
            .run(),
        );
        logger::info_con("HttpServer", format!("HTTP Server started on {}:{}", settings.http.host, settings.http.port).as_str());
        return Ok(HttpClient {});
    }
}
