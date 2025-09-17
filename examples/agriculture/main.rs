//! Agriculture robotics example

use kova_core::{
    init, SensorManager, BlockchainManager, DataValidator,
    sensors::{Camera, LiDAR, IMU, GPS, Thermal, SensorType},
    blockchain::{SolanaClient, IPFSClient},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the core system
    init().await?;
    
    println!("Starting agriculture robotics system...");
    
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
    
    // Setup agricultural robot sensors
    println!("Setting up agricultural robot sensors...");
    
    // High-resolution camera for crop monitoring
    let mut camera_config = kova_core::sensors::CameraConfig::default();
    camera_config.resolution = (3840, 2160); // 4K resolution
    camera_config.frame_rate = 60;
    let camera = Camera::new("crop-camera".to_string(), camera_config)?;
    sensor_manager.add_sensor(Box::new(camera)).await?;
    
    // LiDAR for 3D crop mapping
    let mut lidar_config = kova_core::sensors::LiDARConfig::default();
    lidar_config.range_max = 50.0; // 50m range for field scanning
    lidar_config.angular_resolution = 0.05; // High resolution
    let lidar = LiDAR::new("crop-lidar".to_string(), lidar_config)?;
    sensor_manager.add_sensor(Box::new(lidar)).await?;
    
    // IMU for precise navigation
    let mut imu_config = kova_core::sensors::IMUConfig::default();
    imu_config.sample_rate = 200.0; // High sample rate for precision
    imu_config.calibration_enabled = true;
    let imu = IMU::new("navigation-imu".to_string(), imu_config)?;
    sensor_manager.add_sensor(Box::new(imu)).await?;
    
    // GPS for field positioning
    let mut gps_config = kova_core::sensors::GPSConfig::default();
    gps_config.enable_rtk = true; // RTK GPS for centimeter accuracy
    gps_config.accuracy_threshold = 0.02; // 2cm accuracy
    let gps = GPS::new("rtk-gps".to_string(), gps_config)?;
    sensor_manager.add_sensor(Box::new(gps)).await?;
    
    // Thermal camera for plant health monitoring
    let mut thermal_config = kova_core::sensors::ThermalConfig::default();
    thermal_config.resolution = (160, 120); // High resolution thermal
    thermal_config.temperature_range = (0.0, 100.0); // 0-100°C range
    thermal_config.emissivity = 0.98; // High emissivity for plants
    let thermal = Thermal::new("plant-thermal".to_string(), thermal_config)?;
    sensor_manager.add_sensor(Box::new(thermal)).await?;
    
    println!("Agricultural robot setup complete. Starting field monitoring...");
    
    // Simulate field monitoring for 5 iterations
    for iteration in 0..5 {
        println!("\n--- Field Monitoring Iteration {} ---", iteration + 1);
        
        // Capture data from all sensors
        let sensor_data = sensor_manager.capture_all().await?;
        println!("Captured agricultural data from {} sensors", sensor_data.len());
        
        // Process each sensor's data
        for data in sensor_data {
            println!("Processing data from sensor: {}", data.sensor_id);
            
            // Validate data quality
            let validation_result = validator.validate(&data.data, &data.metadata).await?;
            
            if validation_result.is_valid {
                println!("  Quality score: {:.2}", validation_result.quality_score);
                
                // Store on blockchain
                let hash = blockchain_manager.store_data(&data.data).await?;
                println!("  Agricultural data stored: {}", hash);
                
                // Create contribution
                let contribution = kova_core::blockchain::Contribution {
                    sensor_data_hash: hash,
                    validator_signature: validation_result.signature,
                    timestamp: chrono::Utc::now(),
                    quality_score: validation_result.quality_score,
                    validator_id: "agriculture_validator".to_string(),
                    sensor_id: data.sensor_id,
                };
                
                // Submit contribution
                let tx_hash = blockchain_manager.submit_contribution(&contribution).await?;
                println!("  Contribution submitted: {}", tx_hash);
                
                // Analyze agricultural data
                analyze_agricultural_data(&data, &validation_result).await?;
            } else {
                println!("  Data validation failed: quality score {:.2}", validation_result.quality_score);
            }
        }
        
        // Wait between monitoring cycles
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
    
    println!("\nAgriculture robotics simulation completed successfully!");
    Ok(())
}

/// Analyze agricultural sensor data
async fn analyze_agricultural_data(
    data: &kova_core::sensors::SensorData,
    validation: &kova_core::core::validation::ValidationResult,
) -> Result<(), Box<dyn std::error::Error>> {
    match data.sensor_type {
        SensorType::Camera => {
            println!("    Analyzing crop images...");
            // In a real implementation, this would analyze crop health, growth stage, etc.
            println!("    Crop health assessment: Good");
            println!("    Growth stage: Vegetative");
            println!("    Pest detection: None detected");
        }
        SensorType::LiDAR => {
            println!("    Analyzing 3D crop structure...");
            // In a real implementation, this would analyze plant height, density, etc.
            println!("    Average plant height: 1.2m");
            println!("    Plant density: 85%");
            println!("    Canopy coverage: 78%");
        }
        SensorType::Thermal => {
            println!("    Analyzing thermal data...");
            // In a real implementation, this would analyze plant temperature, stress, etc.
            println!("    Average leaf temperature: 22.5°C");
            println!("    Thermal stress: Low");
            println!("    Water stress: None detected");
        }
        SensorType::GPS => {
            println!("    Analyzing GPS data...");
            // In a real implementation, this would analyze field position, coverage, etc.
            println!("    Field position: Row 15, Plant 8");
            println!("    Coverage progress: 65%");
            println!("    Navigation accuracy: 1.2cm");
        }
        SensorType::IMU => {
            println!("    Analyzing IMU data...");
            // In a real implementation, this would analyze robot orientation, stability, etc.
            println!("    Robot orientation: Stable");
            println!("    Vibration level: Low");
            println!("    Navigation accuracy: High");
        }
    }
    
    Ok(())
}
