//! Error types for the Kova Core system

use thiserror::Error;

/// Result type alias for Kova Core operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for Kova Core
#[derive(Error, Debug)]
pub enum Error {
    /// Sensor-related errors
    #[error("Sensor error: {0}")]
    Sensor(String),

    /// Blockchain-related errors
    #[error("Blockchain error: {0}")]
    Blockchain(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// Network errors
    #[error("Network error: {0}")]
    Network(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Storage errors
    #[error("Storage error: {0}")]
    Storage(String),

    /// ROS2 errors
    #[cfg(feature = "ros2")]
    #[error("ROS2 error: {0}")]
    ROS2(String),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Other errors
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

impl Error {
    /// Create a new sensor error
    pub fn sensor(msg: impl Into<String>) -> Self {
        Self::Sensor(msg.into())
    }

    /// Create a new blockchain error
    pub fn blockchain(msg: impl Into<String>) -> Self {
        Self::Blockchain(msg.into())
    }

    /// Create a new validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    /// Create a new network error
    pub fn network(msg: impl Into<String>) -> Self {
        Self::Network(msg.into())
    }

    /// Create a new configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }

    /// Create a new storage error
    pub fn storage(msg: impl Into<String>) -> Self {
        Self::Storage(msg.into())
    }

    /// Create a new ROS2 error
    #[cfg(feature = "ros2")]
    pub fn ros2(msg: impl Into<String>) -> Self {
        Self::ROS2(msg.into())
    }
}
