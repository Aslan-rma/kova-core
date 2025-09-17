//! Basic robot example

use kova_core::{
    init, SensorManager, BlockchainManager, DataValidator,
    sensors::{Camera, LiDAR, IMU, SensorType},
    blockchain::{SolanaClient, IPFSClient},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the core system
    init().await?;
    
    println!("Starting basic robot example...");
    
    // Create sensor manager
    let mut sensor_manager = SensorManager::new();
    
    // Create blockchain manager
    let mut blockchain_manager = BlockchainManager::new();
    
    // Initialize blockchain clients
    let solana_config = kova_core::blockchain::SolanaConfig::default();
    let solana_client = SolanaClient::new(solana_config).await?;
    blockchain_manager.add_client("solana".to_string(), Box::new(solana_client));
    
    let ipfs_config = kova_core::blockchain::IPFSConfig::default();
    let ipfs_client = IPFSClient::new(ipfs_config).await?;
    blockchain_manager.add_client("ipfs".to_string(), Box::new(ipfs_client));
    
    // Create validator
    let validator = DataValidator::new();
    
    // Setup basic robot sensors
    println!("Setting up basic robot sensors...");
    
    // Camera sensor
    let camera_config = kova_core::sensors::CameraConfig::default();
    let camera = Camera::new("robot-camera".to_string(), camera_config)?;
    sensor_manager.add_sensor(Box::new(camera)).await?;
    
    // LiDAR sensor
    let lidar_config = kova_core::sensors::LiDARConfig::default();
    let lidar = LiDAR::new("robot-lidar".to_string(), lidar_config)?;
    sensor_manager.add_sensor(Box::new(lidar)).await?;
    
    // IMU sensor
    let imu_config = kova_core::sensors::IMUConfig::default();
    let imu = IMU::new("robot-imu".to_string(), imu_config)?;
    sensor_manager.add_sensor(Box::new(imu)).await?;
    
    println!("Basic robot setup complete. Starting data collection...");
    
    // Simulate data collection for 5 iterations
    for iteration in 0..5 {
        println!("\n--- Data Collection Iteration {} ---", iteration + 1);
        
        // Capture data from all sensors
        let sensor_data = sensor_manager.capture_all().await?;
        println!("Captured data from {} sensors", sensor_data.len());
        
        // Process each sensor's data
        for data in sensor_data {
            println!("Processing data from sensor: {}", data.sensor_id);
            
            // Validate data quality
            let validation_result = validator.validate(&data.data, &data.metadata).await?;
            
            if validation_result.is_valid {
                println!("  Quality score: {:.2}", validation_result.quality_score);
                
                // Store on blockchain
                let hash = blockchain_manager.store_data(&data.data).await?;
                println!("  Data stored on blockchain: {}", hash);
                
                // Create contribution
                let contribution = kova_core::blockchain::Contribution {
                    sensor_data_hash: hash,
                    validator_signature: validation_result.signature,
                    timestamp: chrono::Utc::now(),
                    quality_score: validation_result.quality_score,
                    validator_id: "basic_robot_validator".to_string(),
                    sensor_id: data.sensor_id,
                };
                
                // Submit contribution
                let tx_hash = blockchain_manager.submit_contribution(&contribution).await?;
                println!("  Contribution submitted: {}", tx_hash);
            } else {
                println!("  Data validation failed: quality score {:.2}", validation_result.quality_score);
            }
        }
        
        // Wait between iterations
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    
    println!("\nBasic robot example completed successfully!");
    Ok(())
}
