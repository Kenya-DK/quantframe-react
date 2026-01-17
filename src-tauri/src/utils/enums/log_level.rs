use serde::{Deserialize, Serialize};

use crate::utils::modules::logger;

#[derive(PartialEq, Debug, Clone)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
    Trace,
    Critical,
    Unknown(String),
}
impl LogLevel {
    // Create method to convert `OrderType` to a `&str`
    // pub fn as_str(&self) -> &str {
    //     match *self {
    //         LogLevel::Info => "info",
    //         LogLevel::Warning => "warning",
    //         LogLevel::Error => "error",
    //         LogLevel::Debug => "debug",
    //         LogLevel::Trace => "trace",
    //         LogLevel::Critical => "critical",
    //         LogLevel::Unknown(ref i) => i,
    //     }
    // }
}
impl Serialize for LogLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            LogLevel::Info => "info",
            LogLevel::Warning => "warning",
            LogLevel::Error => "error",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
            LogLevel::Critical => "critical",
            LogLevel::Unknown(i) => {
                logger::critical_file(
                    "OrderMode",
                    format!("Unknown OrderMode: {}", i).as_str(),
                    Some("enums.log"),
                );
                "unknown"
            }
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<LogLevel, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "info" => LogLevel::Info,
            "warning" => LogLevel::Warning,
            "error" => LogLevel::Error,
            "debug" => LogLevel::Debug,
            "trace" => LogLevel::Trace,
            "critical" => LogLevel::Critical,
            s => LogLevel::Unknown(s.parse().map_err(|_| {
                serde::de::Error::custom(format!(
                    "invalid value for LogLevel, must be an string: {}",
                    s
                ))
            })?),
        })
    }
}
