//! ROS2 manager

use crate::core::Error;
use crate::robots::ros2::{ROS2Bridge, ROS2Config};

/// ROS2 manager for handling ROS2 operations
pub struct ROS2Manager {
    bridges: std::collections::HashMap<String, ROS2Bridge>,
}

impl ROS2Manager {
    /// Create a new ROS2 manager
    pub fn new() -> Self {
        Self {
            bridges: std::collections::HashMap::new(),
        }
    }

    /// Add a ROS2 bridge
    pub async fn add_bridge(&mut self, name: String, config: ROS2Config) -> Result<(), Error> {
        let bridge = ROS2Bridge::new(config).await?;
        self.bridges.insert(name, bridge);
        Ok(())
    }

    /// Get a ROS2 bridge
    pub fn get_bridge(&self, name: &str) -> Option<&ROS2Bridge> {
        self.bridges.get(name)
    }
}
