use eyre::eyre;
use migration::DbErr;
use regex::Regex;
use reqwest::header::HeaderMap;
use serde_json::{json, Value};

use crate::utils::enums::log_level::LogLevel;

use super::logger::{self, LoggerOptions};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ErrorApiResponse {
    #[serde(rename = "statusCode")]
    pub status_code: i64,

    #[serde(rename = "error")]
    pub error: String,

    #[serde(rename = "messages")]
    pub messages: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "raw_response")]
    pub raw_response: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "url")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "body")]
    pub body: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "method")]
    pub method: Option<String>,
}

#[derive(Debug)]
pub enum ApiResult<T> {
    Success(T, HeaderMap),
    Error(ErrorApiResponse, HeaderMap),
}

#[derive(Debug)]
pub struct AppError {
    component: String,
    eyre_report: String,
    log_level: LogLevel,
}
impl AppError {
    // Custom constructor
    pub fn new(component: &str, eyre_report: eyre::ErrReport) -> Self {
        AppError {
            component: component.to_string(),
            eyre_report: format!("{:?}", eyre_report),
            log_level: LogLevel::Critical,
        }
    }
    pub fn new_db(component: &str, err: DbErr) -> Self {
        AppError {
            component: component.to_string(),
            eyre_report: format!("{:?}", eyre!(err.to_string())),
            log_level: LogLevel::Critical,
        }
    }
    // Custom constructor
    pub fn new_api(
        component: &str,
        mut err: ErrorApiResponse,
        eyre_report: eyre::ErrReport,
        log_level: LogLevel,
    ) -> Self {
        let mut new_err: AppError = AppError::new_with_level(component, eyre_report, log_level);
        let mut cause = new_err.cause();
        let backtrace = new_err.backtrace();
        let mut extra = new_err.extra_data();

        let payload = match err.body {
            Some(mut content) => {
                if content["password"].is_string() {
                    content["password"] = json!("********");
                }
                if content["access_token"].is_string() {
                    content["access_token"] = json!("********");
                }
                if content["email"].is_string() {
                    content["email"] = json!("********");
                }
                if content["check_code"].is_string() {
                    content["check_code"] = json!("********");
                }
                content.clone()
            }
            None => json!({}),
        };
        err.body = Some(payload.clone());
        extra["ApiError"] = json!(err);
        cause = format!(
            "{} The request failed with status code {} to the url: {} with the following message: {}",
            cause,
            err.status_code,
            err.clone().url.unwrap_or("NONE".to_string()),
            err.messages.join(",")
        );
        new_err.eyre_report = format!(
            "{}[J]{}[J]\n\nLocation:\n    {}",
            cause,
            extra.to_string(),
            backtrace
        );
        new_err
    }
    // Custom constructor
    pub fn new_with_level(
        component: &str,
        eyre_report: eyre::ErrReport,
        log_level: LogLevel,
    ) -> Self {
        AppError {
            component: component.to_string(),
            eyre_report: format!("{:?}", eyre_report),
            log_level,
        }
    }
    pub fn get_info(&self) -> (String, String, Value) {
        let e = self.eyre_report.clone();
        // Define the regular expression pattern
        let pattern = r"(.*?)(?:\n\nLocation:\n)(.*)";
        let re = Regex::new(pattern).unwrap();

        // Define a regular expression to match the text between [J] markers
        let json_re = Regex::new(r"\[J\](.*?)\[J\]").unwrap();
        let mut json = json!({});

        for captured in json_re.captures_iter(&e) {
            let json_str = &captured[1];
            match serde_json::from_str::<Value>(json_str) {
                Ok(parsed_json) => {
                    // Merge parsed_json into json
                    for (key, value) in parsed_json.as_object().unwrap() {
                        json[key] = value.clone();
                    }
                }
                Err(err) => {
                    json["ParsingError"] = json!({
                        "message": "Failed to parse the JSON in the error",
                        "error": err.to_string(),
                        "raw": json_str,
                    });
                }
            }
        }
        // Remove the JSONs from the text
        let e = json_re.replace_all(&e, "").to_string();

        // Perform the regex search
        if let Some(captures) = re.captures(&e) {
            let before_location = &captures[1];
            let after_location = &captures[2];
            return (
                before_location.to_string(),
                after_location.to_string(),
                json,
            );
        } else {
            logger::warning(
                "AppError",
                format!(
                    "The regex pattern {} did not match the error string {}",
                    pattern, e
                )
                .as_str(),
                LoggerOptions::default(),
            );
            return ("".to_string(), "".to_string(), json);
        }
    }
    // Getter for component
    pub fn component(&self) -> String {
        self.component.to_string()
    }
    // Getter for component
    pub fn cause(&self) -> String {
        let (before_location, _after_location, _json) = self.get_info();
        before_location
    }
    // Getter for backtrace
    pub fn backtrace(&self) -> String {
        let (_before_location, after_location, _json) = self.get_info();
        after_location.replace("    ", "")
    }
    // Getter for log_level
    pub fn log_level(&self) -> LogLevel {
        self.log_level.clone()
    }
    // Getter for extra_data
    pub fn extra_data(&self) -> Value {
        let (_before_location, _after_location, json) = self.get_info();
        json
    }

    // Getter for component
    pub fn to_json(&self) -> Value {
        json!({
            "component": self.component(),
            "cause": self.cause(),
            "backtrace": self.backtrace(),
            "log_level": self.log_level(),
            "extra_data": self.extra_data(),
        })
    }
}
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.to_json().serialize(serializer)
    }
}

impl<T> From<std::sync::PoisonError<T>> for AppError {
    fn from(e: std::sync::PoisonError<T>) -> Self {
        AppError::new("PoisonError", eyre!(e.to_string()))
    }
}

pub fn create_log_file(file: &str, e: &AppError) {
    let component = e.component();
    let cause = e.cause();
    let backtrace = e.backtrace();
    let log_level = e.log_level();
    let extra = e.extra_data();

    crate::logger::dolog(
        log_level,
        component.as_str(),
        format!(
            "Location: {:?}, {:?}, Extra: <{}>",
            backtrace,
            cause,
            extra.to_string()
        )
        .as_str(),
        LoggerOptions::default().set_file(file),
    );
}
