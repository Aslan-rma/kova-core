//! Advanced fleet management example

use kova_core::{
    init, SensorManager, BlockchainManager, DataValidator,
    sensors::{Camera, LiDAR, IMU, GPS, Thermal, SensorType},
    blockchain::{SolanaClient, IPFSClient, ArweaveClient},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the core system
    init().await?;
    
    println!("Starting advanced fleet management system...");
    
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
    
    let arweave_config = kova_core::blockchain::ArweaveConfig::default();
    let arweave_client = ArweaveClient::new(arweave_config).await?;
    blockchain_manager.add_client("arweave".to_string(), Box::new(arweave_client));
    
    // Create validator
    let validator = DataValidator::new();
    
    // Simulate fleet of robots
    let robot_ids = vec!["robot_001", "robot_002", "robot_003", "robot_004", "robot_005"];
    
    for robot_id in robot_ids {
        println!("Setting up robot: {}", robot_id);
        
        // Add sensors for each robot
        let camera_config = kova_core::sensors::CameraConfig::default();
        let camera = Camera::new(format!("{}-camera", robot_id), camera_config)?;
        sensor_manager.add_sensor(Box::new(camera)).await?;
        
        let lidar_config = kova_core::sensors::LiDARConfig::default();
        let lidar = LiDAR::new(format!("{}-lidar", robot_id), lidar_config)?;
        sensor_manager.add_sensor(Box::new(lidar)).await?;
        
        let imu_config = kova_core::sensors::IMUConfig::default();
        let imu = IMU::new(format!("{}-imu", robot_id), imu_config)?;
        sensor_manager.add_sensor(Box::new(imu)).await?;
        
        let gps_config = kova_core::sensors::GPSConfig::default();
        let gps = GPS::new(format!("{}-gps", robot_id), gps_config)?;
        sensor_manager.add_sensor(Box::new(gps)).await?;
        
        let thermal_config = kova_core::sensors::ThermalConfig::default();
        let thermal = Thermal::new(format!("{}-thermal", robot_id), thermal_config)?;
        sensor_manager.add_sensor(Box::new(thermal)).await?;
    }
    
    println!("Fleet setup complete. Starting data collection...");
    
    // Simulate data collection for 10 iterations
    for iteration in 0..10 {
        println!("\n--- Data Collection Iteration {} ---", iteration + 1);
        
        // Capture data from all sensors
        let sensor_data = sensor_manager.capture_all().await?;
        println!("Captured data from {} sensors", sensor_data.len());
        
        // Process and validate each sensor's data
        for data in sensor_data {
            println!("Processing data from sensor: {}", data.sensor_id);
            
            // Validate data quality
            let validation_result = validator.validate(&data.data, &data.metadata).await?;
            
            if validation_result.is_valid {
                println!("  Quality score: {:.2}", validation_result.quality_score);
                
                // Store on blockchain
                let hash = blockchain_manager.store_data(&data.data).await?;
                println!("  Stored on blockchain: {}", hash);
                
                // Create contribution
                let contribution = kova_core::blockchain::Contribution {
                    sensor_data_hash: hash,
                    validator_signature: validation_result.signature,
                    timestamp: chrono::Utc::now(),
                    quality_score: validation_result.quality_score,
                    validator_id: "fleet_validator".to_string(),
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
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
    
    println!("\nFleet management simulation completed successfully!");
    Ok(())
}
