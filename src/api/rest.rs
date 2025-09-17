//! REST API for Kova Core

use crate::core::Error;
use serde::{Deserialize, Serialize};

/// REST API server
pub struct RestApiServer {
    port: u16,
    host: String,
}

/// API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    /// Create a successful response
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create an error response
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: chrono::Utc::now(),
        }
    }
}

impl RestApiServer {
    /// Create a new REST API server
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    /// Start the server
    pub async fn start(&self) -> Result<(), Error> {
        tracing::info!("Starting REST API server on {}:{}", self.host, self.port);
        // Implementation would go here
        Ok(())
    }
}
