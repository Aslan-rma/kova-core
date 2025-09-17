//! LiDAR sensor implementation

use crate::core::Error;
use crate::sensors::{Sensor, SensorData, SensorType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// LiDAR sensor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiDARConfig {
    /// Minimum range in meters
    pub range_min: f32,
    /// Maximum range in meters
    pub range_max: f32,
    /// Angular resolution in degrees
    pub angular_resolution: f32,
    /// Scan frequency in Hz
    pub scan_frequency: f32,
    /// Point cloud format
    pub point_cloud_format: PointCloudFormat,
    /// Number of laser beams
    pub laser_count: u32,
    /// Vertical field of view in degrees
    pub vertical_fov: f32,
    /// Horizontal field of view in degrees
    pub horizontal_fov: f32,
}

/// Point cloud format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PointCloudFormat {
    /// XYZ format (x, y, z)
    XYZ,
    /// XYZI format (x, y, z, intensity)
    XYZI,
    /// XYZRGB format (x, y, z, r, g, b)
    XYZRGB,
    /// XYZIR format (x, y, z, intensity, ring)
    XYZIR,
}

impl Default for LiDARConfig {
    fn default() -> Self {
        Self {
            range_min: 0.1,
            range_max: 100.0,
            angular_resolution: 0.1,
            scan_frequency: 10.0,
            point_cloud_format: PointCloudFormat::XYZI,
            laser_count: 16,
            vertical_fov: 30.0,
            horizontal_fov: 360.0,
        }
    }
}

/// Point structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub intensity: Option<f32>,
    pub ring: Option<u32>,
}

/// LiDAR sensor implementation
pub struct LiDAR {
    id: String,
    config: LiDARConfig,
    is_initialized: bool,
}

impl LiDAR {
    /// Create a new LiDAR sensor
    pub fn new(id: String, config: LiDARConfig) -> Result<Self, Error> {
        Ok(Self {
            id,
            config,
            is_initialized: false,
        })
    }

    /// Initialize the LiDAR
    pub async fn initialize(&mut self) -> Result<(), Error> {
        tracing::info!("Initializing LiDAR: {}", self.id);
        self.is_initialized = true;
        Ok(())
    }

    /// Capture a point cloud
    pub async fn capture(&mut self) -> Result<Vec<Point>, Error> {
        if !self.is_initialized {
            return Err(Error::sensor("LiDAR not initialized"));
        }

        self.generate_test_point_cloud().await
    }

    /// Generate test point cloud
    async fn generate_test_point_cloud(&self) -> Result<Vec<Point>, Error> {
        let mut points = Vec::new();
        
        // Generate points in a spiral pattern
        let num_points = 1000;
        for i in 0..num_points {
            let angle = (i as f32 / num_points as f32) * 2.0 * std::f32::consts::PI;
            let radius = self.config.range_min + (i as f32 / num_points as f32) * (self.config.range_max - self.config.range_min);
            
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            let z = (i as f32 / num_points as f32) * 2.0 - 1.0; // -1 to 1
            
            let intensity = Some((i as f32 / num_points as f32) * 255.0);
            let ring = Some(i % self.config.laser_count);
            
            points.push(Point {
                x,
                y,
                z,
                intensity,
                ring,
            });
        }
        
        Ok(points)
    }

    /// Serialize point cloud to bytes
    pub fn serialize_point_cloud(&self, points: &[Point]) -> Result<Vec<u8>, Error> {
        let mut data = Vec::new();
        
        for point in points {
            match self.config.point_cloud_format {
                PointCloudFormat::XYZ => {
                    data.extend_from_slice(&point.x.to_le_bytes());
                    data.extend_from_slice(&point.y.to_le_bytes());
                    data.extend_from_slice(&point.z.to_le_bytes());
                }
                PointCloudFormat::XYZI => {
                    data.extend_from_slice(&point.x.to_le_bytes());
                    data.extend_from_slice(&point.y.to_le_bytes());
                    data.extend_from_slice(&point.z.to_le_bytes());
                    data.extend_from_slice(&point.intensity.unwrap_or(0.0).to_le_bytes());
                }
                PointCloudFormat::XYZRGB => {
                    data.extend_from_slice(&point.x.to_le_bytes());
                    data.extend_from_slice(&point.y.to_le_bytes());
                    data.extend_from_slice(&point.z.to_le_bytes());
                    data.push(255); // R
                    data.push(128); // G
                    data.push(64);  // B
                }
                PointCloudFormat::XYZIR => {
                    data.extend_from_slice(&point.x.to_le_bytes());
                    data.extend_from_slice(&point.y.to_le_bytes());
                    data.extend_from_slice(&point.z.to_le_bytes());
                    data.extend_from_slice(&point.intensity.unwrap_or(0.0).to_le_bytes());
                    data.extend_from_slice(&point.ring.unwrap_or(0).to_le_bytes());
                }
            }
        }
        
        Ok(data)
    }

    /// Get LiDAR configuration
    pub fn config(&self) -> &LiDARConfig {
        &self.config
    }

    /// Update LiDAR configuration
    pub fn update_config(&mut self, config: LiDARConfig) {
        self.config = config;
    }
}

impl Sensor for LiDAR {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn sensor_type(&self) -> SensorType {
        SensorType::LiDAR
    }
    
    async fn capture(&mut self) -> Result<SensorData, Error> {
        let points = self.capture().await?;
        let data = self.serialize_point_cloud(&points)?;
        
        let mut metadata = HashMap::new();
        metadata.insert("point_count".to_string(), points.len().to_string());
        metadata.insert("range_min".to_string(), self.config.range_min.to_string());
        metadata.insert("range_max".to_string(), self.config.range_max.to_string());
        metadata.insert("format".to_string(), format!("{:?}", self.config.point_cloud_format));
        
        Ok(SensorData {
            sensor_id: self.id.clone(),
            sensor_type: SensorType::LiDAR,
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
