use eyre::eyre;
use regex::Regex;
use serde_json::{json, Value};

use crate::logger::LogLevel;
#[derive(Debug)]
pub struct AppError {
    component: &'static str,
    eyre_report: eyre::ErrReport,
    log_level: LogLevel,
}
impl AppError {
    // Custom constructor
    pub fn new(component: &'static str, eyre_report: eyre::ErrReport) -> Self {
        AppError {
            component,
            eyre_report,
            log_level: LogLevel::Critical,
        }
    }
    // Custom constructor
    pub fn new_with_level(
        component: &'static str,
        eyre_report: eyre::ErrReport,
        log_level: LogLevel,
    ) -> Self {
        AppError {
            component,
            eyre_report,
            log_level,
        }
    }
    pub fn get_info(&self, e: String) -> (String, String, Value) {
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
                    json["error"] = json!(err.to_string());
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
            println!("Pattern not found in the text: {}.", e);
            return ("".to_string(), "".to_string(), json);
        }
    }
    // Getter for component
    pub fn component(&self) -> String {
        self.component.to_string()
    }
    // Getter for component
    pub fn cause(&self) -> String {
        let (before_location, _after_location, _json) =
            self.get_info(format!("{:?}", self.eyre_report));
        before_location
    }
    // Getter for backtrace
    pub fn backtrace(&self) -> String {
        let (_before_location, after_location, _json) =
            self.get_info(format!("{:?}", self.eyre_report));
        after_location.replace("    ", "")
    }
    // Getter for log_level
    pub fn log_level(&self) -> LogLevel {
        self.log_level.clone()
    }
    // Getter for extra_data
    pub fn extra_data(&self) -> Value {
        let (_before_location, _after_location, json) =
            self.get_info(format!("{:?}", self.eyre_report));
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
        return AppError::serialize(self, serializer);
    }
}

impl<T> From<std::sync::PoisonError<T>> for AppError {
    fn from(e: std::sync::PoisonError<T>) -> Self {
        AppError::new("PoisonError", eyre!(e.to_string()))
    }
}

pub fn create_log_file(file: String, e: &AppError) {
    let component = e.component();
    let cause = e.cause();
    let backtrace = e.backtrace();
    let log_level = e.log_level();
    let extra = e.extra_data();
    crate::logger::dolog(
        log_level,
        component.as_str(),
        format!("Location: {:?}, {:?}, {:?}", backtrace, cause, extra).as_str(),
        true,
        Some(file.as_str()),
    );
}
