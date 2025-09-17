//! # Kova Core
//!
//! Core library for the Kova decentralized robotics data network.
//!
//! This library provides essential functionality for:
//! - Multi-sensor data processing and validation
//! - Blockchain integration (Solana, Arweave, IPFS)
//! - ROS2 bridge for robotics systems
//! - Distributed validation protocols
//! - Cross-platform robotics support
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use kova_core::{init, SensorManager, BlockchainClient};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     init().await?;
//!     
//!     let mut sensor_manager = SensorManager::new();
//!     let blockchain_client = BlockchainClient::new().await?;
//!     
//!     // Your robotics application code here
//!     
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod api;
pub mod blockchain;
pub mod core;
pub mod robots;
pub mod sdk;
pub mod sensors;

/// Initialize the Kova Core system
pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    tracing::info!("Initializing Kova Core");
    Ok(())
}

/// Re-export commonly used types
pub use core::{
    config::Config,
    error::{Error, Result},
    network::NetworkManager,
    protocol::ProtocolManager,
    rewards::RewardManager,
    storage::StorageManager,
    validation::ValidationManager,
};

/// Re-export sensor types
#[cfg(feature = "sensors")]
pub use sensors::{
    camera::Camera,
    gps::GPS,
    imu::IMU,
    lidar::LiDAR,
    thermal::Thermal,
    manager::SensorManager,
};

/// Re-export blockchain types
#[cfg(feature = "blockchain")]
pub use blockchain::{
    arweave::ArweaveClient,
    ipfs::IPFSClient,
    solana::SolanaClient,
    manager::BlockchainManager,
};

/// Re-export ROS2 types
#[cfg(feature = "ros2")]
pub use robots::ros2::{
    bridge::ROS2Bridge,
    config::ROS2Config,
    manager::ROS2Manager,
};

/// Re-export validation types
#[cfg(feature = "validation")]
pub use core::validation::{
    DataValidator,
    ValidationConfig,
    ValidationResult,
    QualityMetrics,
};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Build information
pub const BUILD_INFO: &str = concat!(
    "Kova Core ",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("GIT_HASH"),
    ")"
);