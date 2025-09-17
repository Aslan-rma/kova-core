//! Thermal sensor implementation

use crate::core::Error;
use crate::sensors::{Sensor, SensorData, SensorType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Thermal sensor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalConfig {
    /// Image resolution (width, height)
    pub resolution: (u32, u32),
    /// Temperature range in Celsius
    pub temperature_range: (f32, f32),
    /// Emissivity (0.0 to 1.0)
    pub emissivity: f32,
    /// Distance to target in meters
    pub distance: f32,
    /// Relative humidity (0.0 to 1.0)
    pub humidity: f32,
    /// Atmospheric temperature in Celsius
    pub atmospheric_temp: f32,
    /// Enable calibration
    pub enable_calibration: bool,
    /// Calibration data
    pub calibration_data: Option<CalibrationData>,
}

/// Calibration data for thermal sensor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationData {
    pub offset: f32,
    pub gain: f32,
    pub dead_pixels: Vec<(u32, u32)>,
    pub temperature_lut: Vec<f32>,
}

/// Thermal data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalData {
    /// Temperature map in Celsius
    pub temperature_map: Vec<Vec<f32>>,
    /// Minimum temperature
    pub min_temperature: f32,
    /// Maximum temperature
    pub max_temperature: f32,
    /// Average temperature
    pub avg_temperature: f32,
    /// Hot spots (x, y, temperature)
    pub hot_spots: Vec<(u32, u32, f32)>,
    /// Cold spots (x, y, temperature)
    pub cold_spots: Vec<(u32, u32, f32)>,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Default for ThermalConfig {
    fn default() -> Self {
        Self {
            resolution: (80, 60),
            temperature_range: (-40.0, 120.0),
            emissivity: 0.95,
            distance: 1.0,
            humidity: 0.5,
            atmospheric_temp: 20.0,
            enable_calibration: true,
            calibration_data: None,
        }
    }
}

/// Thermal sensor implementation
pub struct Thermal {
    id: String,
    config: ThermalConfig,
    is_initialized: bool,
}

impl Thermal {
    /// Create a new thermal sensor
    pub fn new(id: String, config: ThermalConfig) -> Result<Self, Error> {
        Ok(Self {
            id,
            config,
            is_initialized: false,
        })
    }

    /// Initialize the thermal sensor
    pub async fn initialize(&mut self) -> Result<(), Error> {
        tracing::info!("Initializing thermal sensor: {}", self.id);
        
        if self.config.enable_calibration {
            self.calibrate().await?;
        }
        
        self.is_initialized = true;
        Ok(())
    }

    /// Calibrate the thermal sensor
    pub async fn calibrate(&mut self) -> Result<(), Error> {
        tracing::info!("Calibrating thermal sensor: {}", self.id);
        
        // Simulate calibration process
        let calibration_data = CalibrationData {
            offset: 0.5,
            gain: 1.02,
            dead_pixels: vec![(10, 15), (25, 30), (40, 45)],
            temperature_lut: (0..256)
                .map(|i| (i as f32 - 128.0) * 0.1 + 20.0)
                .collect(),
        };
        
        self.config.calibration_data = Some(calibration_data);
        Ok(())
    }

    /// Capture thermal data
    pub async fn capture(&mut self) -> Result<ThermalData, Error> {
        if !self.is_initialized {
            return Err(Error::sensor("Thermal sensor not initialized"));
        }

        self.generate_test_thermal_data().await
    }

    /// Generate test thermal data
    async fn generate_test_thermal_data(&self) -> Result<ThermalData, Error> {
        let timestamp = chrono::Utc::now();
        let time = timestamp.timestamp_millis() as f32 / 1000.0;
        
        let (width, height) = self.config.resolution;
        let mut temperature_map = vec![vec![0.0; width as usize]; height as usize];
        
        // Generate thermal pattern
        for y in 0..height {
            for x in 0..width {
                let center_x = width as f32 / 2.0;
                let center_y = height as f32 / 2.0;
                
                let distance = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
                
                // Base temperature with circular gradient
                let base_temp = 25.0 + 10.0 * (1.0 - distance / (width as f32 / 2.0));
                
                // Add time-varying heat sources
                let heat_source_1 = 15.0 * (time * 0.5 + x as f32 * 0.1).sin();
                let heat_source_2 = 8.0 * (time * 0.3 + y as f32 * 0.15).cos();
                
                // Add noise
                let noise = 2.0 * (time * 2.0 + x as f32 + y as f32).sin();
                
                let temperature = base_temp + heat_source_1 + heat_source_2 + noise;
                
                // Apply temperature range limits
                let temperature = temperature
                    .max(self.config.temperature_range.0)
                    .min(self.config.temperature_range.1);
                
                temperature_map[y as usize][x as usize] = temperature;
            }
        }
        
        // Apply calibration if available
        if let Some(calibration) = &self.config.calibration_data {
            self.apply_calibration(&mut temperature_map, calibration);
        }
        
        // Find temperature statistics
        let mut min_temp = f32::MAX;
        let mut max_temp = f32::MIN;
        let mut sum_temp = 0.0;
        let mut count = 0;
        
        for row in &temperature_map {
            for &temp in row {
                min_temp = min_temp.min(temp);
                max_temp = max_temp.max(temp);
                sum_temp += temp;
                count += 1;
            }
        }
        
        let avg_temp = sum_temp / count as f32;
        
        // Find hot and cold spots
        let hot_spots = self.find_hot_spots(&temperature_map, avg_temp + 5.0);
        let cold_spots = self.find_cold_spots(&temperature_map, avg_temp - 5.0);
        
        Ok(ThermalData {
            temperature_map,
            min_temperature: min_temp,
            max_temperature: max_temp,
            avg_temperature: avg_temp,
            hot_spots,
            cold_spots,
            timestamp,
        })
    }

    /// Apply calibration to temperature map
    fn apply_calibration(&self, temperature_map: &mut Vec<Vec<f32>>, calibration: &CalibrationData) {
        for row in temperature_map.iter_mut() {
            for temp in row.iter_mut() {
                *temp = (*temp + calibration.offset) * calibration.gain;
            }
        }
        
        // Handle dead pixels
        for &(x, y) in &calibration.dead_pixels {
            if y < temperature_map.len() as u32 && x < temperature_map[y as usize].len() as u32 {
                // Interpolate from surrounding pixels
                let interpolated = self.interpolate_dead_pixel(temperature_map, x as usize, y as usize);
                temperature_map[y as usize][x as usize] = interpolated;
            }
        }
    }

    /// Interpolate dead pixel value
    fn interpolate_dead_pixel(&self, temperature_map: &Vec<Vec<f32>>, x: usize, y: usize) -> f32 {
        let mut sum = 0.0;
        let mut count = 0;
        
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && ny >= 0 
                    && ny < temperature_map.len() as i32 
                    && nx < temperature_map[ny as usize].len() as i32 {
                    sum += temperature_map[ny as usize][nx as usize];
                    count += 1;
                }
            }
        }
        
        if count > 0 {
            sum / count as f32
        } else {
            20.0 // Default temperature
        }
    }

    /// Find hot spots in temperature map
    fn find_hot_spots(&self, temperature_map: &Vec<Vec<f32>>, threshold: f32) -> Vec<(u32, u32, f32)> {
        let mut hot_spots = Vec::new();
        
        for (y, row) in temperature_map.iter().enumerate() {
            for (x, &temp) in row.iter().enumerate() {
                if temp > threshold {
                    hot_spots.push((x as u32, y as u32, temp));
                }
            }
        }
        
        // Sort by temperature (hottest first)
        hot_spots.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        hot_spots.truncate(10); // Keep top 10 hot spots
        
        hot_spots
    }

    /// Find cold spots in temperature map
    fn find_cold_spots(&self, temperature_map: &Vec<Vec<f32>>, threshold: f32) -> Vec<(u32, u32, f32)> {
        let mut cold_spots = Vec::new();
        
        for (y, row) in temperature_map.iter().enumerate() {
            for (x, &temp) in row.iter().enumerate() {
                if temp < threshold {
                    cold_spots.push((x as u32, y as u32, temp));
                }
            }
        }
        
        // Sort by temperature (coldest first)
        cold_spots.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
        cold_spots.truncate(10); // Keep top 10 cold spots
        
        cold_spots
    }

    /// Serialize thermal data to bytes
    pub fn serialize_thermal_data(&self, thermal_data: &ThermalData) -> Result<Vec<u8>, Error> {
        let mut data = Vec::new();
        
        // Serialize resolution
        data.extend_from_slice(&thermal_data.temperature_map[0].len().to_le_bytes());
        data.extend_from_slice(&thermal_data.temperature_map.len().to_le_bytes());
        
        // Serialize temperature map
        for row in &thermal_data.temperature_map {
            for &temp in row {
                data.extend_from_slice(&temp.to_le_bytes());
            }
        }
        
        // Serialize statistics
        data.extend_from_slice(&thermal_data.min_temperature.to_le_bytes());
        data.extend_from_slice(&thermal_data.max_temperature.to_le_bytes());
        data.extend_from_slice(&thermal_data.avg_temperature.to_le_bytes());
        
        // Serialize hot spots
        data.extend_from_slice(&thermal_data.hot_spots.len().to_le_bytes());
        for (x, y, temp) in &thermal_data.hot_spots {
            data.extend_from_slice(&x.to_le_bytes());
            data.extend_from_slice(&y.to_le_bytes());
            data.extend_from_slice(&temp.to_le_bytes());
        }
        
        // Serialize cold spots
        data.extend_from_slice(&thermal_data.cold_spots.len().to_le_bytes());
        for (x, y, temp) in &thermal_data.cold_spots {
            data.extend_from_slice(&x.to_le_bytes());
            data.extend_from_slice(&y.to_le_bytes());
            data.extend_from_slice(&temp.to_le_bytes());
        }
        
        // Serialize timestamp
        let timestamp_bytes = thermal_data.timestamp.timestamp_millis().to_le_bytes();
        data.extend_from_slice(&timestamp_bytes);
        
        Ok(data)
    }

    /// Get thermal configuration
    pub fn config(&self) -> &ThermalConfig {
        &self.config
    }

    /// Update thermal configuration
    pub fn update_config(&mut self, config: ThermalConfig) {
        self.config = config;
    }
}

impl Sensor for Thermal {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn sensor_type(&self) -> SensorType {
        SensorType::Thermal
    }
    
    async fn capture(&mut self) -> Result<SensorData, Error> {
        let thermal_data = self.capture().await?;
        let data = self.serialize_thermal_data(&thermal_data)?;
        
        let mut metadata = HashMap::new();
        metadata.insert("resolution".to_string(), format!("{}x{}", self.config.resolution.0, self.config.resolution.1));
        metadata.insert("min_temperature".to_string(), thermal_data.min_temperature.to_string());
        metadata.insert("max_temperature".to_string(), thermal_data.max_temperature.to_string());
        metadata.insert("avg_temperature".to_string(), thermal_data.avg_temperature.to_string());
        metadata.insert("hot_spots_count".to_string(), thermal_data.hot_spots.len().to_string());
        metadata.insert("cold_spots_count".to_string(), thermal_data.cold_spots.len().to_string());
        metadata.insert("emissivity".to_string(), self.config.emissivity.to_string());
        
        Ok(SensorData {
            sensor_id: self.id.clone(),
            sensor_type: SensorType::Thermal,
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
