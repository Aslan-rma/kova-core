//! Camera sensor implementation

use crate::core::Error;
use crate::sensors::{Sensor, SensorData, SensorType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Camera sensor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    /// Image resolution (width, height)
    pub resolution: (u32, u32),
    /// Frame rate in FPS
    pub frame_rate: u32,
    /// Image format
    pub format: ImageFormat,
    /// Auto exposure enabled
    pub auto_exposure: bool,
    /// Auto white balance enabled
    pub auto_white_balance: bool,
    /// Exposure compensation (-2.0 to 2.0)
    pub exposure_compensation: f32,
    /// ISO sensitivity
    pub iso_sensitivity: u32,
    /// Focus mode
    pub focus_mode: FocusMode,
    /// White balance mode
    pub white_balance_mode: WhiteBalanceMode,
}

/// Image format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFormat {
    /// RGB format
    RGB,
    /// RGBA format
    RGBA,
    /// Grayscale format
    Grayscale,
    /// YUV format
    YUV,
    /// JPEG format
    JPEG,
    /// PNG format
    PNG,
}

/// Focus mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FocusMode {
    /// Auto focus
    Auto,
    /// Manual focus
    Manual,
    /// Continuous focus
    Continuous,
    /// Fixed focus
    Fixed,
}

/// White balance mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhiteBalanceMode {
    /// Auto white balance
    Auto,
    /// Daylight
    Daylight,
    /// Cloudy
    Cloudy,
    /// Tungsten
    Tungsten,
    /// Fluorescent
    Fluorescent,
    /// Manual
    Manual,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            resolution: (1920, 1080),
            frame_rate: 30,
            format: ImageFormat::RGB,
            auto_exposure: true,
            auto_white_balance: true,
            exposure_compensation: 0.0,
            iso_sensitivity: 100,
            focus_mode: FocusMode::Auto,
            white_balance_mode: WhiteBalanceMode::Auto,
        }
    }
}

/// Camera sensor implementation
pub struct Camera {
    id: String,
    config: CameraConfig,
    is_initialized: bool,
}

impl Camera {
    /// Create a new camera sensor
    pub fn new(id: String, config: CameraConfig) -> Result<Self, Error> {
        Ok(Self {
            id,
            config,
            is_initialized: false,
        })
    }

    /// Initialize the camera
    pub async fn initialize(&mut self) -> Result<(), Error> {
        tracing::info!("Initializing camera: {}", self.id);
        self.is_initialized = true;
        Ok(())
    }

    /// Capture an image
    pub async fn capture(&mut self) -> Result<Vec<u8>, Error> {
        if !self.is_initialized {
            return Err(Error::sensor("Camera not initialized"));
        }

        // Simulate image capture
        let image_data = self.generate_test_image().await?;
        Ok(image_data)
    }

    /// Generate test image data
    async fn generate_test_image(&self) -> Result<Vec<u8>, Error> {
        let (width, height) = self.config.resolution;
        let pixel_count = (width * height) as usize;
        
        match self.config.format {
            ImageFormat::RGB => {
                let mut data = vec![0u8; pixel_count * 3];
                for i in 0..pixel_count {
                    let base = i * 3;
                    data[base] = (i % 256) as u8;     // Red
                    data[base + 1] = ((i * 2) % 256) as u8; // Green
                    data[base + 2] = ((i * 3) % 256) as u8; // Blue
                }
                Ok(data)
            }
            ImageFormat::RGBA => {
                let mut data = vec![0u8; pixel_count * 4];
                for i in 0..pixel_count {
                    let base = i * 4;
                    data[base] = (i % 256) as u8;     // Red
                    data[base + 1] = ((i * 2) % 256) as u8; // Green
                    data[base + 2] = ((i * 3) % 256) as u8; // Blue
                    data[base + 3] = 255; // Alpha
                }
                Ok(data)
            }
            ImageFormat::Grayscale => {
                let data = (0..pixel_count)
                    .map(|i| (i % 256) as u8)
                    .collect();
                Ok(data)
            }
            _ => Err(Error::sensor("Unsupported image format")),
        }
    }

    /// Get camera configuration
    pub fn config(&self) -> &CameraConfig {
        &self.config
    }

    /// Update camera configuration
    pub fn update_config(&mut self, config: CameraConfig) {
        self.config = config;
    }
}

impl Sensor for Camera {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn sensor_type(&self) -> SensorType {
        SensorType::Camera
    }
    
    async fn capture(&mut self) -> Result<SensorData, Error> {
        let data = self.capture().await?;
        let mut metadata = HashMap::new();
        metadata.insert("resolution".to_string(), format!("{}x{}", self.config.resolution.0, self.config.resolution.1));
        metadata.insert("format".to_string(), format!("{:?}", self.config.format));
        metadata.insert("frame_rate".to_string(), self.config.frame_rate.to_string());
        
        Ok(SensorData {
            sensor_id: self.id.clone(),
            sensor_type: SensorType::Camera,
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
