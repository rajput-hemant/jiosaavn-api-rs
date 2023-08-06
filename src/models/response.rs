use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum StatusCode {
    Success,
    Failed,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomResponse<T> {
    pub status: StatusCode,
    pub message: &'static str,
    pub data: Option<T>,
}

impl<T> CustomResponse<T> {
    pub fn new(status: StatusCode, message: &'static str, data: Option<T>) -> Self {
        Self {
            status,
            message,
            data,
        }
    }
}
