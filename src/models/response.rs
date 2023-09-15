use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Success,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CResponse<T> {
    pub status: Status,
    pub message: &'static str,
    pub data: Option<T>,
}

impl<T> CResponse<T> {
    pub fn new(status: Status, message: &'static str, data: Option<T>) -> Self {
        Self {
            status,
            message,
            data,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StatusCode {
    Success,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
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
