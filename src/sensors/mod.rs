//! Sensor processing and management

pub mod camera;
pub mod gps;
pub mod imu;
pub mod lidar;
pub mod thermal;
pub mod manager;

pub use manager::SensorManager;

/// Common sensor types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SensorType {
    /// Camera sensor
    Camera,
    /// LiDAR sensor
    LiDAR,
    /// IMU sensor
    IMU,
    /// GPS sensor
    GPS,
    /// Thermal sensor
    Thermal,
}

/// Sensor data structure
#[derive(Debug, Clone)]
pub struct SensorData {
    /// Sensor ID
    pub sensor_id: String,
    /// Sensor type
    pub sensor_type: SensorType,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Raw data
    pub data: Vec<u8>,
    /// Metadata
    pub metadata: std::collections::HashMap<String, String>,
}

/// Sensor trait
pub trait Sensor: Send + Sync {
    /// Get sensor ID
    fn id(&self) -> &str;
    
    /// Get sensor type
    fn sensor_type(&self) -> SensorType;
    
    /// Capture data from sensor
    async fn capture(&mut self) -> Result<SensorData, crate::core::Error>;
    
    /// Check if sensor is available
    async fn is_available(&self) -> bool;
    
    /// Get sensor configuration
    fn config(&self) -> &dyn std::fmt::Debug;
}
