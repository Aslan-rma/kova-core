//! Sensor manager for handling multiple sensors

use crate::core::Error;
use crate::sensors::{Sensor, SensorData, SensorType};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Sensor manager for handling multiple sensors
pub struct SensorManager {
    sensors: RwLock<HashMap<String, Box<dyn Sensor>>>,
}

impl SensorManager {
    /// Create a new sensor manager
    pub fn new() -> Self {
        Self {
            sensors: RwLock::new(HashMap::new()),
        }
    }

    /// Add a sensor to the manager
    pub async fn add_sensor(&self, sensor: Box<dyn Sensor>) -> Result<(), Error> {
        let sensor_id = sensor.id().to_string();
        let mut sensors = self.sensors.write().await;
        sensors.insert(sensor_id, sensor);
        Ok(())
    }

    /// Remove a sensor from the manager
    pub async fn remove_sensor(&self, sensor_id: &str) -> Result<(), Error> {
        let mut sensors = self.sensors.write().await;
        sensors.remove(sensor_id);
        Ok(())
    }

    /// Get a sensor by ID
    pub async fn get_sensor(&self, sensor_id: &str) -> Option<Box<dyn Sensor>> {
        let sensors = self.sensors.read().await;
        // Note: This is a simplified implementation
        // In practice, you'd need to handle the trait object properly
        None
    }

    /// Capture data from all sensors
    pub async fn capture_all(&self) -> Result<Vec<SensorData>, Error> {
        let sensors = self.sensors.read().await;
        let mut results = Vec::new();

        for (sensor_id, sensor) in sensors.iter() {
            if sensor.is_available().await {
                match sensor.capture().await {
                    Ok(data) => results.push(data),
                    Err(e) => {
                        tracing::warn!("Failed to capture data from sensor {}: {}", sensor_id, e);
                    }
                }
            }
        }

        Ok(results)
    }

    /// Capture data from a specific sensor
    pub async fn capture_sensor(&self, sensor_id: &str) -> Result<SensorData, Error> {
        let sensors = self.sensors.read().await;
        let sensor = sensors.get(sensor_id)
            .ok_or_else(|| Error::sensor(format!("Sensor {} not found", sensor_id)))?;
        
        sensor.capture().await
    }

    /// Get list of available sensors
    pub async fn list_sensors(&self) -> Vec<String> {
        let sensors = self.sensors.read().await;
        sensors.keys().cloned().collect()
    }

    /// Get sensors by type
    pub async fn get_sensors_by_type(&self, sensor_type: SensorType) -> Vec<String> {
        let sensors = self.sensors.read().await;
        sensors.iter()
            .filter(|(_, sensor)| sensor.sensor_type() == sensor_type)
            .map(|(id, _)| id.clone())
            .collect()
    }
}
