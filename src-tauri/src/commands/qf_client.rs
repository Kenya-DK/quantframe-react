use std::sync::{Arc, Mutex};

use serde_json::Value;

use crate::{
    qf_client::client::QFClient,
    utils::{
        enums::log_level::LogLevel,
        modules::{
            error::{ApiResult, AppError},
            logger,
        },
    },
};

#[tauri::command]
pub async fn qf_post(
    url: &str,
    body: Value,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<Value, AppError> {
    let qf = qf.lock().expect("Could not lock qf").clone();

    match qf.post::<Value>(url, body).await {
        Ok(ApiResult::Success(data, _)) => {
            return Ok(data);
        }
        Ok(ApiResult::Error(e, _headers)) => {
            let err = qf.create_api_error(
                "QFClient:Post",
                e,
                eyre::eyre!("There was an error posting data"),
                LogLevel::Critical,
            );
            return Err(err);
        }
        Err(e) => return Err(e),
    };
}
#[tauri::command]
pub async fn qf_get(
    url: &str,
    qf: tauri::State<'_, Arc<Mutex<QFClient>>>,
) -> Result<Value, AppError> {
    let qf = qf.lock().expect("Could not lock qf").clone();

    match qf.get::<Value>(url, false).await {
        Ok(ApiResult::Success(data, _)) => {
            logger::log_json("data.json", &data).unwrap();
            return Ok(data);
        }
        Ok(ApiResult::Error(e, _headers)) => {
            let err = qf.create_api_error(
                "QFClient:Get",
                e,
                eyre::eyre!("There was an error fetching data"),
                LogLevel::Critical,
            );
            return Err(err);
        }
        Err(e) => return Err(e),
    };
}
