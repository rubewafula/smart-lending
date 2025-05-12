use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;
use regex::Regex;

use rand::RngCore;
use rand::rngs::OsRng;
use hex;


#[derive(Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
    status: u16,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, axum::Json(self)).into_response()
    }
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: Option<T>) -> Self {
        Self {
            success: true,
            data,
            message: None,
            status: StatusCode::OK.as_u16(),
        }
    }

    pub fn error(msg: &str, status: u16) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(msg.to_string()),
            status,
        }
    }
}

pub fn is_valid_phone(phone: &str) -> Option<String> {
    let re = Regex::new(r"^(?:|\+|254)?([71]\d{8})$").unwrap();

    if let Some(captures) = re.captures(phone) {
        if let Some(matched) = captures.get(1) {
            // Normalize to E.164 format without "+"
            return Some(format!("254{}", matched.as_str()));
        }
    }
    None
}

pub fn is_valid_email(email: &str) -> bool {
    let re = Regex::new(
        r"(?i)^[a-z0-9_.+-]+@[a-z0-9-]+\.[a-z0-9-.]+$"
    ).unwrap();
    re.is_match(email)
}

pub fn generate_token_128() -> String {
    let mut bytes = [0u8; 16]; // 128 bits = 16 bytes
    OsRng.fill_bytes(&mut bytes); // Uses secure randomness
    hex::encode(bytes) // Convert to a 32-char hex string
}
