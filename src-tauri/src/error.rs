use eyre::eyre;
use regex::Regex;
use serde_json::{json, Value};

use crate::logger::LogLevel;

#[derive(Debug)]
pub struct AppError(pub &'static str, pub eyre::ErrReport);

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        return AppError::serialize(self, serializer);
    }
}

impl<T> From<std::sync::PoisonError<T>> for AppError {
    fn from(e: std::sync::PoisonError<T>) -> Self {
        AppError("PoisonError", eyre!(e.to_string()))
    }
}

pub trait GetErrorInfo {
    fn component(&self) -> String;
    fn cause(&self) -> String;
    fn backtrace(&self) -> String;
    fn log_level(&self) -> LogLevel;
    fn extra_data(&self) -> Value;
    fn to_json(&self) -> Value;
}

pub fn get_info(e: String) -> (String, String, Value) {
    // Define the regular expression pattern
    let pattern = r"(.*?)(?:\n\nLocation:\n)(.*)";
    let re = Regex::new(pattern).unwrap();

    // Define a regular expression to match the text between [J] markers
    let json_re = Regex::new(r"\[J\](.*?)\[J\]").unwrap();
    let mut json = json!({});
    // Get JSON from the text
    if let Some(captured) = json_re.captures(e.as_str()) {
        // Extract the captured text
        let json_str = &captured[1];
        // Parse the captured JSON string into a serde_json::Value
        match serde_json::from_str(json_str) {
            Ok(parsed_json) => {
                json = parsed_json;
            }
            Err(err) => {
                json = json!({"error": err.to_string()});
            }
        }
    }

    // Remove the JSON from the text
    let e = json_re.replace_all(e.as_str(), "").to_string();

    // Perform the regex search
    if let Some(captures) = re.captures(e.as_str()) {
        let before_location = &captures[1];
        let after_location = &captures[2];
        return (
            before_location.to_string(),
            after_location.to_string(),
            json,
        );
    } else {
        println!("Pattern not found in the text:{:?}.", e);
        return ("".to_string(), "".to_string(), json);
    }
}

impl GetErrorInfo for AppError {
    fn component(&self) -> String {
        self.0.to_string()
    }
    fn cause(&self) -> String {
        let (before_location, _after_location, _json) = get_info(format!("{:?}", self.1));
        before_location
    }
    fn backtrace(&self) -> String {
        let (_before_location, after_location, _json) = get_info(format!("{:?}", self.1));
        after_location.replace("    ", "")
    }
    fn extra_data(&self) -> Value {
        let (_before_location, _after_location, json) = get_info(format!("{:?}", self.1));
        json
    }
    fn log_level(&self) -> LogLevel {
        LogLevel::Critical
    }
    fn to_json(&self) -> Value {
        json!({
            "component": self.component(),
            "cause": self.cause(),
            "backtrace": self.backtrace(),
            "log_level": self.log_level(),
            "extra_data": self.extra_data(),
        })
    }
}

pub fn create_log_file(file: String, e: &AppError) {
    let component = e.component();
    let cause = e.cause();
    let backtrace = e.backtrace();
    let log_level = e.log_level();
    crate::logger::dolog(
        log_level,
        component.as_str(),
        format!("Error: {:?}, {:?}", backtrace, cause).as_str(),
        true,
        Some(file.as_str()),
    );
}
