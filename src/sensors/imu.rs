//! IMU sensor implementation

use crate::core::Error;
use crate::sensors::{Sensor, SensorData, SensorType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// IMU sensor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IMUConfig {
    /// Sample rate in Hz
    pub sample_rate: f32,
    /// Accelerometer range in g
    pub accelerometer_range: f32,
    /// Gyroscope range in degrees per second
    pub gyroscope_range: f32,
    /// Magnetometer enabled
    pub magnetometer_enabled: bool,
    /// Temperature compensation enabled
    pub temperature_compensation: bool,
    /// Noise filtering enabled
    pub noise_filtering: bool,
    /// Calibration enabled
    pub calibration_enabled: bool,
}

/// IMU data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IMUData {
    /// Linear acceleration (x, y, z) in m/s²
    pub linear_acceleration: [f32; 3],
    /// Angular velocity (x, y, z) in rad/s
    pub angular_velocity: [f32; 3],
    /// Magnetic field (x, y, z) in µT
    pub magnetic_field: Option<[f32; 3]>,
    /// Temperature in Celsius
    pub temperature: Option<f32>,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Default for IMUConfig {
    fn default() -> Self {
        Self {
            sample_rate: 100.0,
            accelerometer_range: 16.0,
            gyroscope_range: 2000.0,
            magnetometer_enabled: true,
            temperature_compensation: true,
            noise_filtering: true,
            calibration_enabled: true,
        }
    }
}

/// IMU sensor implementation
pub struct IMU {
    id: String,
    config: IMUConfig,
    is_initialized: bool,
    calibration_data: Option<CalibrationData>,
}

/// Calibration data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationData {
    pub accelerometer_bias: [f32; 3],
    pub gyroscope_bias: [f32; 3],
    pub magnetometer_bias: [f32; 3],
    pub accelerometer_scale: [f32; 3],
    pub gyroscope_scale: [f32; 3],
    pub magnetometer_scale: [f32; 3],
}

impl IMU {
    /// Create a new IMU sensor
    pub fn new(id: String, config: IMUConfig) -> Result<Self, Error> {
        Ok(Self {
            id,
            config,
            is_initialized: false,
            calibration_data: None,
        })
    }

    /// Initialize the IMU
    pub async fn initialize(&mut self) -> Result<(), Error> {
        tracing::info!("Initializing IMU: {}", self.id);
        
        if self.config.calibration_enabled {
            self.calibrate().await?;
        }
        
        self.is_initialized = true;
        Ok(())
    }

    /// Calibrate the IMU
    pub async fn calibrate(&mut self) -> Result<(), Error> {
        tracing::info!("Calibrating IMU: {}", self.id);
        
        // Simulate calibration process
        let calibration_data = CalibrationData {
            accelerometer_bias: [0.1, -0.05, 0.02],
            gyroscope_bias: [0.01, 0.02, -0.01],
            magnetometer_bias: [10.0, -5.0, 8.0],
            accelerometer_scale: [1.0, 1.0, 1.0],
            gyroscope_scale: [1.0, 1.0, 1.0],
            magnetometer_scale: [1.0, 1.0, 1.0],
        };
        
        self.calibration_data = Some(calibration_data);
        Ok(())
    }

    /// Capture IMU data
    pub async fn capture(&mut self) -> Result<IMUData, Error> {
        if !self.is_initialized {
            return Err(Error::sensor("IMU not initialized"));
        }

        self.generate_test_imu_data().await
    }

    /// Generate test IMU data
    async fn generate_test_imu_data(&self) -> Result<IMUData, Error> {
        let timestamp = chrono::Utc::now();
        
        // Generate realistic IMU data
        let time = timestamp.timestamp_millis() as f32 / 1000.0;
        
        // Simulate gravity + small movements
        let linear_acceleration = [
            0.0 + 0.1 * (time * 0.5).sin(),
            0.0 + 0.05 * (time * 0.3).cos(),
            9.81 + 0.2 * (time * 0.7).sin(),
        ];
        
        // Simulate rotation
        let angular_velocity = [
            0.1 * (time * 0.2).sin(),
            0.05 * (time * 0.4).cos(),
            0.02 * (time * 0.6).sin(),
        ];
        
        // Simulate magnetic field
        let magnetic_field = if self.config.magnetometer_enabled {
            Some([
                25.0 + 2.0 * (time * 0.1).sin(),
                5.0 + 1.0 * (time * 0.15).cos(),
                45.0 + 3.0 * (time * 0.08).sin(),
            ])
        } else {
            None
        };
        
        // Simulate temperature
        let temperature = if self.config.temperature_compensation {
            Some(25.0 + 2.0 * (time * 0.01).sin())
        } else {
            None
        };
        
        let mut imu_data = IMUData {
            linear_acceleration,
            angular_velocity,
            magnetic_field,
            temperature,
            timestamp,
        };
        
        // Apply calibration if available
        if let Some(calibration) = &self.calibration_data {
            self.apply_calibration(&mut imu_data, calibration);
        }
        
        // Apply noise filtering if enabled
        if self.config.noise_filtering {
            self.apply_noise_filtering(&mut imu_data);
        }
        
        Ok(imu_data)
    }

    /// Apply calibration to IMU data
    fn apply_calibration(&self, imu_data: &mut IMUData, calibration: &CalibrationData) {
        // Apply accelerometer calibration
        for i in 0..3 {
            imu_data.linear_acceleration[i] = 
                (imu_data.linear_acceleration[i] - calibration.accelerometer_bias[i]) 
                * calibration.accelerometer_scale[i];
        }
        
        // Apply gyroscope calibration
        for i in 0..3 {
            imu_data.angular_velocity[i] = 
                (imu_data.angular_velocity[i] - calibration.gyroscope_bias[i]) 
                * calibration.gyroscope_scale[i];
        }
        
        // Apply magnetometer calibration
        if let Some(ref mut mag) = imu_data.magnetic_field {
            for i in 0..3 {
                mag[i] = (mag[i] - calibration.magnetometer_bias[i]) 
                    * calibration.magnetometer_scale[i];
            }
        }
    }

    /// Apply noise filtering to IMU data
    fn apply_noise_filtering(&self, _imu_data: &mut IMUData) {
        // Simple low-pass filter simulation
        // In a real implementation, this would use proper filtering algorithms
    }

    /// Serialize IMU data to bytes
    pub fn serialize_imu_data(&self, imu_data: &IMUData) -> Result<Vec<u8>, Error> {
        let mut data = Vec::new();
        
        // Serialize linear acceleration
        for &acc in &imu_data.linear_acceleration {
            data.extend_from_slice(&acc.to_le_bytes());
        }
        
        // Serialize angular velocity
        for &gyro in &imu_data.angular_velocity {
            data.extend_from_slice(&gyro.to_le_bytes());
        }
        
        // Serialize magnetic field if available
        if let Some(mag) = imu_data.magnetic_field {
            for &field in &mag {
                data.extend_from_slice(&field.to_le_bytes());
            }
        }
        
        // Serialize temperature if available
        if let Some(temp) = imu_data.temperature {
            data.extend_from_slice(&temp.to_le_bytes());
        }
        
        // Serialize timestamp
        let timestamp_bytes = imu_data.timestamp.timestamp_millis().to_le_bytes();
        data.extend_from_slice(&timestamp_bytes);
        
        Ok(data)
    }

    /// Get IMU configuration
    pub fn config(&self) -> &IMUConfig {
        &self.config
    }

    /// Update IMU configuration
    pub fn update_config(&mut self, config: IMUConfig) {
        self.config = config;
    }
}

impl Sensor for IMU {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn sensor_type(&self) -> SensorType {
        SensorType::IMU
    }
    
    async fn capture(&mut self) -> Result<SensorData, Error> {
        let imu_data = self.capture().await?;
        let data = self.serialize_imu_data(&imu_data)?;
        
        let mut metadata = HashMap::new();
        metadata.insert("sample_rate".to_string(), self.config.sample_rate.to_string());
        metadata.insert("accelerometer_range".to_string(), self.config.accelerometer_range.to_string());
        metadata.insert("gyroscope_range".to_string(), self.config.gyroscope_range.to_string());
        metadata.insert("magnetometer_enabled".to_string(), self.config.magnetometer_enabled.to_string());
        metadata.insert("calibrated".to_string(), self.calibration_data.is_some().to_string());
        
        Ok(SensorData {
            sensor_id: self.id.clone(),
            sensor_type: SensorType::IMU,
            timestamp: chrono::Utc::now(),
            data,
            metadata,
        })
    }
    
    async fn is_available(&self) -> bool {
        self.is_initialized
    }
    
    fn config(&self) -> &dyn std::fmt::Debug {
        &self.config
    }
}
