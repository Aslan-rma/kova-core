//! GPS sensor implementation

use crate::core::Error;
use crate::sensors::{Sensor, SensorData, SensorType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GPS sensor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPSConfig {
    /// Update rate in Hz
    pub update_rate: f32,
    /// Accuracy threshold in meters
    pub accuracy_threshold: f32,
    /// Enable DGPS
    pub enable_dgps: bool,
    /// Enable RTK
    pub enable_rtk: bool,
    /// Reference ellipsoid
    pub reference_ellipsoid: String,
    /// Coordinate system
    pub coordinate_system: CoordinateSystem,
}

/// Coordinate system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordinateSystem {
    /// WGS84
    WGS84,
    /// NAD83
    NAD83,
    /// ETRS89
    ETRS89,
    /// Custom
    Custom,
}

/// GPS data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPSData {
    /// Latitude in degrees
    pub latitude: f64,
    /// Longitude in degrees
    pub longitude: f64,
    /// Altitude in meters
    pub altitude: f64,
    /// Accuracy in meters
    pub accuracy: f64,
    /// Speed in m/s
    pub speed: f64,
    /// Heading in degrees
    pub heading: f64,
    /// Number of satellites
    pub satellite_count: u32,
    /// Fix quality
    pub fix_quality: FixQuality,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// GPS fix quality
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FixQuality {
    /// No fix
    NoFix,
    /// GPS fix
    GPSFix,
    /// DGPS fix
    DGPSFix,
    /// RTK fix
    RTKFix,
    /// RTK float
    RTKFloat,
}

impl Default for GPSConfig {
    fn default() -> Self {
        Self {
            update_rate: 1.0,
            accuracy_threshold: 5.0,
            enable_dgps: false,
            enable_rtk: false,
            reference_ellipsoid: "WGS84".to_string(),
            coordinate_system: CoordinateSystem::WGS84,
        }
    }
}

/// GPS sensor implementation
pub struct GPS {
    id: String,
    config: GPSConfig,
    is_initialized: bool,
    last_position: Option<(f64, f64, f64)>,
}

impl GPS {
    /// Create a new GPS sensor
    pub fn new(id: String, config: GPSConfig) -> Result<Self, Error> {
        Ok(Self {
            id,
            config,
            is_initialized: false,
            last_position: None,
        })
    }

    /// Initialize the GPS
    pub async fn initialize(&mut self) -> Result<(), Error> {
        tracing::info!("Initializing GPS: {}", self.id);
        self.is_initialized = true;
        Ok(())
    }

    /// Capture GPS data
    pub async fn capture(&mut self) -> Result<GPSData, Error> {
        if !self.is_initialized {
            return Err(Error::sensor("GPS not initialized"));
        }

        self.generate_test_gps_data().await
    }

    /// Generate test GPS data
    async fn generate_test_gps_data(&self) -> Result<GPSData, Error> {
        let timestamp = chrono::Utc::now();
        let time = timestamp.timestamp_millis() as f64 / 1000.0;
        
        // Simulate movement around a base location
        let base_lat = 37.7749; // San Francisco
        let base_lon = -122.4194;
        let base_alt = 10.0;
        
        // Add small circular movement
        let radius = 0.001; // ~100m
        let latitude = base_lat + radius * (time * 0.1).cos();
        let longitude = base_lon + radius * (time * 0.1).sin();
        let altitude = base_alt + 2.0 * (time * 0.05).sin();
        
        // Simulate accuracy based on configuration
        let accuracy = if self.config.enable_rtk {
            0.01 + 0.005 * (time * 0.2).sin() // RTK accuracy
        } else if self.config.enable_dgps {
            0.5 + 0.2 * (time * 0.3).sin() // DGPS accuracy
        } else {
            2.0 + 1.0 * (time * 0.4).sin() // Standard GPS accuracy
        };
        
        // Simulate speed and heading
        let speed = 1.0 + 0.5 * (time * 0.15).sin(); // 1-1.5 m/s
        let heading = (time * 10.0) % 360.0; // Rotating heading
        
        // Simulate satellite count
        let satellite_count = if self.config.enable_rtk {
            12 + ((time * 0.1) as u32 % 4) // 12-15 satellites
        } else {
            8 + ((time * 0.2) as u32 % 4) // 8-11 satellites
        };
        
        // Determine fix quality
        let fix_quality = if self.config.enable_rtk {
            if accuracy < 0.02 {
                FixQuality::RTKFix
            } else {
                FixQuality::RTKFloat
            }
        } else if self.config.enable_dgps {
            FixQuality::DGPSFix
        } else {
            FixQuality::GPSFix
        };
        
        Ok(GPSData {
            latitude,
            longitude,
            altitude,
            accuracy,
            speed,
            heading,
            satellite_count,
            fix_quality,
            timestamp,
        })
    }

    /// Convert to UTM coordinates
    pub fn to_utm(&self, gps_data: &GPSData) -> Result<(f64, f64, f64), Error> {
        // Simplified UTM conversion
        // In a real implementation, this would use proper UTM conversion algorithms
        let utm_x = (gps_data.longitude + 180.0) * 100000.0;
        let utm_y = (gps_data.latitude + 90.0) * 100000.0;
        let utm_z = gps_data.altitude;
        
        Ok((utm_x, utm_y, utm_z))
    }

    /// Convert to local coordinates
    pub fn to_local(&self, gps_data: &GPSData, reference: (f64, f64, f64)) -> Result<(f64, f64, f64), Error> {
        let (ref_lat, ref_lon, ref_alt) = reference;
        
        // Convert to meters using simple approximation
        let lat_diff = gps_data.latitude - ref_lat;
        let lon_diff = gps_data.longitude - ref_lon;
        let alt_diff = gps_data.altitude - ref_alt;
        
        // Approximate conversion to meters
        let x = lon_diff * 111320.0 * gps_data.latitude.to_radians().cos();
        let y = lat_diff * 110540.0;
        let z = alt_diff;
        
        Ok((x, y, z))
    }

    /// Calculate distance between two GPS points
    pub fn calculate_distance(&self, point1: (f64, f64), point2: (f64, f64)) -> f64 {
        let (lat1, lon1) = point1;
        let (lat2, lon2) = point2;
        
        // Haversine formula
        let dlat = (lat2 - lat1).to_radians();
        let dlon = (lon2 - lon1).to_radians();
        
        let a = (dlat / 2.0).sin().powi(2) + 
                lat1.to_radians().cos() * lat2.to_radians().cos() * 
                (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().asin();
        
        6371000.0 * c // Earth radius in meters
    }

    /// Serialize GPS data to bytes
    pub fn serialize_gps_data(&self, gps_data: &GPSData) -> Result<Vec<u8>, Error> {
        let mut data = Vec::new();
        
        // Serialize coordinates
        data.extend_from_slice(&gps_data.latitude.to_le_bytes());
        data.extend_from_slice(&gps_data.longitude.to_le_bytes());
        data.extend_from_slice(&gps_data.altitude.to_le_bytes());
        
        // Serialize accuracy and speed
        data.extend_from_slice(&gps_data.accuracy.to_le_bytes());
        data.extend_from_slice(&gps_data.speed.to_le_bytes());
        data.extend_from_slice(&gps_data.heading.to_le_bytes());
        
        // Serialize satellite count and fix quality
        data.extend_from_slice(&gps_data.satellite_count.to_le_bytes());
        data.push(gps_data.fix_quality as u8);
        
        // Serialize timestamp
        let timestamp_bytes = gps_data.timestamp.timestamp_millis().to_le_bytes();
        data.extend_from_slice(&timestamp_bytes);
        
        Ok(data)
    }

    /// Get GPS configuration
    pub fn config(&self) -> &GPSConfig {
        &self.config
    }

    /// Update GPS configuration
    pub fn update_config(&mut self, config: GPSConfig) {
        self.config = config;
    }
}

impl Sensor for GPS {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn sensor_type(&self) -> SensorType {
        SensorType::GPS
    }
    
    async fn capture(&mut self) -> Result<SensorData, Error> {
        let gps_data = self.capture().await?;
        let data = self.serialize_gps_data(&gps_data)?;
        
        let mut metadata = HashMap::new();
        metadata.insert("latitude".to_string(), gps_data.latitude.to_string());
        metadata.insert("longitude".to_string(), gps_data.longitude.to_string());
        metadata.insert("altitude".to_string(), gps_data.altitude.to_string());
        metadata.insert("accuracy".to_string(), gps_data.accuracy.to_string());
        metadata.insert("satellite_count".to_string(), gps_data.satellite_count.to_string());
        metadata.insert("fix_quality".to_string(), format!("{:?}", gps_data.fix_quality));
        
        Ok(SensorData {
            sensor_id: self.id.clone(),
            sensor_type: SensorType::GPS,
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
