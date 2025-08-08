use serde_json::{Error, json};
use std::fmt::{Display, Formatter};

use crate::errors::*;

#[derive(Debug)]
pub enum ApiError {
    TooManyRequests(RequestError),
    RequestError(RequestError),
    Unauthorized(RequestError),
    ParsingError(RequestError, Error),
    NotFound(RequestError),
    BadRequest(RequestError),
    InvalidCredentials(RequestError),
    Forbidden(RequestError),
    UserBanned(RequestError),
    Unknown(String),
    InvalidType { expected: String, found: String },
}
impl ApiError {
    pub fn mask_sensitive_data(&mut self, properties: &[&str]) {
        match self {
            ApiError::TooManyRequests(req_err) => req_err.mask_sensitive_data(properties),
            ApiError::RequestError(req_err) => req_err.mask_sensitive_data(properties),
            ApiError::Unauthorized(req_err) => req_err.mask_sensitive_data(properties),
            ApiError::ParsingError(req_err, _) => req_err.mask_sensitive_data(properties),
            ApiError::NotFound(req_err) => req_err.mask_sensitive_data(properties),
            ApiError::BadRequest(req_err) => req_err.mask_sensitive_data(properties),
            ApiError::InvalidCredentials(req_err) => req_err.mask_sensitive_data(properties),
            ApiError::Forbidden(req_err) => req_err.mask_sensitive_data(properties),
            ApiError::UserBanned(req_err) => req_err.mask_sensitive_data(properties),
            ApiError::Unknown(_) | ApiError::InvalidType { .. } => {}
        }
    }
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            ApiError::TooManyRequests(req_err) => json!({
                "type": "TooManyRequests",
                "error": req_err,
            }),
            ApiError::RequestError(req_err) => json!({
                "type": "RequestError",
                "error": req_err,
            }),
            ApiError::Unauthorized(req_err) => json!({
                "type": "Unauthorized",
                "error": req_err,
            }),
            ApiError::ParsingError(req_err, parse_err) => json!({
                "type": "ParsingError",
                "error": req_err,
                "parse_error": parse_err.to_string(),
            }),
            ApiError::NotFound(req_err) => json!({
                "type": "NotFound",
                "error": req_err,
            }),
            ApiError::BadRequest(req_err) => json!({
                "type": "BadRequest",
                "error": req_err,
            }),
            ApiError::InvalidCredentials(req_err) => json!({
                "type": "InvalidCredentials",
                "error": req_err,
            }),
            ApiError::Forbidden(req_err) => json!({
                "type": "Forbidden",
                "error": req_err,
            }),
            ApiError::Unknown(_) => json!({
                "type": "Unknown",
                "message": "An unknown error occurred."
            }),
            ApiError::InvalidType { expected, found } => json!({
                "type": "InvalidType",
                "expected": expected,
                "found": found,
            }),
            ApiError::UserBanned(req_err) => json!({
                "type": "UserBanned",
                "error": req_err,
            }),
        }
    }
}
impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::TooManyRequests(req_err) => {
                write!(f, "Too many requests: {}", req_err.error_sentence())
            }
            ApiError::RequestError(req_err) => {
                write!(f, "Request error: {}", req_err.error_sentence())
            }
            ApiError::Unauthorized(req_err) => {
                write!(f, "Unauthorized: {}", req_err.error_sentence())
            }
            ApiError::ParsingError(req_err, parse_err) => {
                write!(
                    f,
                    "Parsing error: {} - {}",
                    req_err.error_sentence(),
                    parse_err
                )
            }
            ApiError::NotFound(req_err) => {
                write!(f, "Not found: {}", req_err.error_sentence())
            }
            ApiError::BadRequest(req_err) => {
                write!(f, "Bad request: {}", req_err.error_sentence())
            }
            ApiError::InvalidCredentials(req_err) => {
                write!(f, "Invalid credentials: {}", req_err.error_sentence())
            }
            ApiError::Forbidden(req_err) => {
                write!(f, "Forbidden: {}", req_err.error_sentence())
            }
            ApiError::Unknown(msg) => {
                write!(f, "Unknown error: {}", msg)
            }
            ApiError::InvalidType { expected, found } => {
                write!(
                    f,
                    "Invalid type: expected '{}', found '{}'",
                    expected, found
                )
            }
            ApiError::UserBanned(req_err) => {
                write!(f, "User banned: {}", req_err.error_sentence())
            }
        }
    }
}
