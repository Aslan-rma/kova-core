//! Unit tests for sensor modules

use kova_core::{
    sensors::{Camera, LiDAR, IMU, GPS, Thermal, SensorType, SensorData},
    core::Error,
};

#[tokio::test]
async fn test_camera_sensor() {
    let config = kova_core::sensors::CameraConfig::default();
    let mut camera = Camera::new("test-camera".to_string(), config).unwrap();
    
    // Test sensor initialization
    assert_eq!(camera.id(), "test-camera");
    assert_eq!(camera.sensor_type(), SensorType::Camera);
    
    // Test data capture
    let sensor_data = camera.capture().await.unwrap();
    assert_eq!(sensor_data.sensor_id, "test-camera");
    assert_eq!(sensor_data.sensor_type, SensorType::Camera);
    assert!(!sensor_data.data.is_empty());
}

#[tokio::test]
async fn test_lidar_sensor() {
    let config = kova_core::sensors::LiDARConfig::default();
    let mut lidar = LiDAR::new("test-lidar".to_string(), config).unwrap();
    
    // Test sensor initialization
    assert_eq!(lidar.id(), "test-lidar");
    assert_eq!(lidar.sensor_type(), SensorType::LiDAR);
    
    // Test data capture
    let sensor_data = lidar.capture().await.unwrap();
    assert_eq!(sensor_data.sensor_id, "test-lidar");
    assert_eq!(sensor_data.sensor_type, SensorType::LiDAR);
    assert!(!sensor_data.data.is_empty());
}

#[tokio::test]
async fn test_imu_sensor() {
    let config = kova_core::sensors::IMUConfig::default();
    let mut imu = IMU::new("test-imu".to_string(), config).unwrap();
    
    // Test sensor initialization
    assert_eq!(imu.id(), "test-imu");
    assert_eq!(imu.sensor_type(), SensorType::IMU);
    
    // Test data capture
    let sensor_data = imu.capture().await.unwrap();
    assert_eq!(sensor_data.sensor_id, "test-imu");
    assert_eq!(sensor_data.sensor_type, SensorType::IMU);
    assert!(!sensor_data.data.is_empty());
}

#[tokio::test]
async fn test_gps_sensor() {
    let config = kova_core::sensors::GPSConfig::default();
    let mut gps = GPS::new("test-gps".to_string(), config).unwrap();
    
    // Test sensor initialization
    assert_eq!(gps.id(), "test-gps");
    assert_eq!(gps.sensor_type(), SensorType::GPS);
    
    // Test data capture
    let sensor_data = gps.capture().await.unwrap();
    assert_eq!(sensor_data.sensor_id, "test-gps");
    assert_eq!(sensor_data.sensor_type, SensorType::GPS);
    assert!(!sensor_data.data.is_empty());
}

#[tokio::test]
async fn test_thermal_sensor() {
    let config = kova_core::sensors::ThermalConfig::default();
    let mut thermal = Thermal::new("test-thermal".to_string(), config).unwrap();
    
    // Test sensor initialization
    assert_eq!(thermal.id(), "test-thermal");
    assert_eq!(thermal.sensor_type(), SensorType::Thermal);
    
    // Test data capture
    let sensor_data = thermal.capture().await.unwrap();
    assert_eq!(sensor_data.sensor_id, "test-thermal");
    assert_eq!(sensor_data.sensor_type, SensorType::Thermal);
    assert!(!sensor_data.data.is_empty());
}

#[tokio::test]
async fn test_sensor_availability() {
    let config = kova_core::sensors::CameraConfig::default();
    let camera = Camera::new("test-camera".to_string(), config).unwrap();
    
    // Test availability before initialization
    assert!(!camera.is_available().await);
}
