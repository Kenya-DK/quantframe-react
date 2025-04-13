
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use crate::utils::modules::{
        error::AppError,
        logger::{self, LoggerOptions},
        states,
    };

use super::modules::stock::{add_item, add_riven};
#[derive(Clone, Debug)]
pub struct HttpClient {}

impl HttpClient {
    pub fn setup() -> Result<Self, AppError> {
        let settings = states::settings()?;
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
                    .service(web::scope("/stock").service(add_riven).service(add_item))
            })
            .bind((settings.http.clone().host, settings.http.port as u16))
            .map_err(|e| AppError::new("HttpServer", eyre::eyre!(e)))?
            .run(),
        );
        logger::info(
            "HttpServer",
            format!(
                "HTTP Server started on {}:{}",
                settings.http.host, settings.http.port
            )
            .as_str(),
            LoggerOptions::default(),
        );
        return Ok(HttpClient {});
    }
}
