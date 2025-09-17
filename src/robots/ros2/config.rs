//! ROS2 configuration

use serde::{Deserialize, Serialize};

/// ROS2 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ROS2Config {
    /// Node name
    pub node_name: String,
    /// Namespace
    pub namespace: String,
    /// QoS profile
    pub qos_profile: String,
}

impl Default for ROS2Config {
    fn default() -> Self {
        Self {
            node_name: "kova_bridge".to_string(),
            namespace: "/kova".to_string(),
            qos_profile: "default".to_string(),
        }
    }
}
