//! Basic usage example for Kova Core

use kova_core::{
    init, SensorManager, BlockchainManager, DataValidator,
    sensors::{Camera, LiDAR, IMU},
    blockchain::{SolanaClient, IPFSClient},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the core system
    init().await?;
    
    // Create sensor manager
    let mut sensor_manager = SensorManager::new();
    
    // Add sensors (simplified - actual implementation would require proper sensor types)
    // let camera = Camera::new("camera-001", CameraConfig::default())?;
    // let lidar = LiDAR::new("lidar-001", LiDARConfig::default())?;
    // let imu = IMU::new("imu-001", IMUConfig::default())?;
    
    // sensor_manager.add_sensor(Box::new(camera)).await?;
    // sensor_manager.add_sensor(Box::new(lidar)).await?;
    // sensor_manager.add_sensor(Box::new(imu)).await?;
    
    // Create blockchain manager
    let mut blockchain_manager = BlockchainManager::new();
    
    // Add blockchain clients
    // let solana_client = SolanaClient::new().await?;
    // let ipfs_client = IPFSClient::new().await?;
    
    // blockchain_manager.add_client("solana".to_string(), Box::new(solana_client));
    // blockchain_manager.add_client("ipfs".to_string(), Box::new(ipfs_client));
    
    // Create validator
    let validator = DataValidator::new();
    
    println!("Kova Core initialized successfully!");
    println!("Sensor manager created with {} sensors", sensor_manager.list_sensors().await.len());
    
    Ok(())
}
