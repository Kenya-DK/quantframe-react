use eyre::eyre;
use regex::Regex;

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
}

pub fn get_info(e: String) -> (String, String) {
    // Define the regular expression pattern
    let pattern = r"(.*?)(?:\n\nLocation:\n)(.*)";
    let re = Regex::new(pattern).unwrap();
    // Perform the regex search
    if let Some(captures) = re.captures(e.as_str()) {
        let before_location = &captures[1];
        let after_location = &captures[2];
        return (before_location.to_string(), after_location.to_string());
    } else {
        println!("Pattern not found in the text:{:?}.", e);
        return ("".to_string(), "".to_string());
    }
}

impl GetErrorInfo for AppError {
    fn component(&self) -> String {
        self.0.to_string()
    }
    fn cause(&self) -> String {
        let (before_location, _after_location) = get_info(format!("{:?}", self.1));
        before_location
    }
    fn backtrace(&self) -> String {
        let (_before_location, after_location) = get_info(format!("{:?}", self.1));
        after_location.replace("    ", "")
    }
    fn log_level(&self) -> LogLevel {
        LogLevel::Critical
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
