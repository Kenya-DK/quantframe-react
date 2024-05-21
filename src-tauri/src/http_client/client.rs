use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};

use crate::{
    settings::SettingsState, utils::modules::error::AppError,
};

use super::modules::stock::{add_item, add_riven};

#[derive(Clone, Debug)]
pub struct HttpClient {}

impl HttpClient {
    pub fn setup(settings: Arc<Mutex<SettingsState>>) -> Result<Self, AppError> {
        let settings = settings.lock().unwrap();
        tauri::async_runtime::spawn(
            HttpServer::new(|| {
                App::new().service(web::scope("/stock").service(add_riven).service(add_item))
            })
            .bind((settings.http.clone().host, settings.http.port as u16))
            .map_err(|e| AppError::new("HttpServer", eyre::eyre!(e)))?
            .run(),
        );
        return Ok(HttpClient {});
    }
}
