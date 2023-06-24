use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerResponse<T> {
    pub error: bool,
    pub message: Option<String>, // if error=true, contains the error message if any
    pub data: Option<T>,
}

impl<T> ServerResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            error: false,
            message: None,
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            error: true,
            message: Some(message),
            data: None,
        }
    }
}
