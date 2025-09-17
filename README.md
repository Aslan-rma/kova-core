# Kova Core

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Documentation](https://docs.rs/kova-core/badge.svg)](https://docs.rs/kova-core)
[![Build Status](https://github.com/kovasystems/kova-core/workflows/CI/badge.svg)](https://github.com/kovasystems/kova-core/actions)

**Core library for the Kova decentralized robotics data network**

Kova Core is the foundational Rust library that powers the Kova ecosystem, providing essential functionality for sensor data processing, blockchain integration, distributed validation, and robotics middleware integration.

## Features

- **Multi-Sensor Support**: Camera, LiDAR, IMU, GPS, and thermal imaging
- **Blockchain Integration**: Solana, Arweave, and IPFS support
- **ROS2 Bridge**: Native ROS2 integration for robotics systems
- **Data Validation**: Comprehensive validation and quality assessment
- **Distributed Protocols**: Proof of Sensory Contribution (PoSC) implementation
- **Cross-Platform**: Support for various robot platforms and operating systems
- **Async/Await**: Built on Tokio for high-performance async operations

## Quick Start

### Installation

Add Kova Core to your `Cargo.toml`:

```toml
[dependencies]
kova-core = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Usage

```rust
use kova_core::{
    init, SensorManager, BlockchainClient, ValidationEngine,
    sensors::{Camera, LiDAR, IMU},
    blockchain::{SolanaClient, IPFSClient},
    validation::DataValidator,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the core system
    init().await?;
    
    // Create sensor manager
    let mut sensor_manager = SensorManager::new();
    
    // Add sensors
    let camera = Camera::new("camera-001", CameraConfig::default())?;
    let lidar = LiDAR::new("lidar-001", LiDARConfig::default())?;
    let imu = IMU::new("imu-001", IMUConfig::default())?;
    
    sensor_manager.add_sensor(camera).await?;
    sensor_manager.add_sensor(lidar).await?;
    sensor_manager.add_sensor(imu).await?;
    
    // Process sensor data
    let sensor_data = sensor_manager.capture_all().await?;
    
    // Validate data quality
    let validator = DataValidator::new();
    let validation_result = validator.validate(&sensor_data).await?;
    
    // Store on blockchain
    let blockchain_client = SolanaClient::new().await?;
    let ipfs_client = IPFSClient::new().await?;
    
    let contribution = Contribution::new(sensor_data, validation_result);
    let tx_hash = blockchain_client.submit_contribution(&contribution).await?;
    
    println!("Contribution submitted: {}", tx_hash);
    Ok(())
}
```

## Architecture

### Core Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Sensor Layer  │───▶│  Processing     │───▶│  Validation     │
│                 │    │  Pipeline       │    │  Engine         │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   ROS2 Bridge   │    │  Data Storage   │    │  Blockchain     │
│                 │    │  (IPFS/Arweave) │    │  Integration    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Sensor Processing Pipeline

1. **Data Capture**: Raw sensor data collection
2. **Preprocessing**: Noise reduction, calibration, format conversion
3. **Validation**: Quality assessment and anomaly detection
4. **Storage**: Distributed storage on IPFS/Arweave
5. **Blockchain**: Transaction submission and reward distribution

## API Reference

### Sensor Management

```rust
use kova_core::sensors::{SensorManager, SensorType, SensorConfig};

let mut manager = SensorManager::new();

// Add camera sensor
let camera_config = CameraConfig {
    resolution: (1920, 1080),
    frame_rate: 30,
    format: ImageFormat::RGB,
    auto_exposure: true,
    auto_white_balance: true,
};

let camera = Camera::new("camera-001", camera_config)?;
manager.add_sensor(camera).await?;

// Capture data from all sensors
let data = manager.capture_all().await?;
```

### Blockchain Integration

```rust
use kova_core::blockchain::{SolanaClient, IPFSClient, ArweaveClient};

// Initialize blockchain clients
let solana = SolanaClient::new().await?;
let ipfs = IPFSClient::new().await?;
let arweave = ArweaveClient::new().await?;

// Store data on IPFS
let ipfs_hash = ipfs.store_data(&sensor_data).await?;

// Submit to Solana
let contribution = Contribution {
    sensor_data_hash: ipfs_hash,
    validator_signature: validation_result.signature,
    timestamp: chrono::Utc::now(),
    quality_score: validation_result.quality_score,
};

let tx_hash = solana.submit_contribution(&contribution).await?;
```

### Data Validation

```rust
use kova_core::validation::{DataValidator, ValidationConfig, QualityMetrics};

let validator = DataValidator::new();

let config = ValidationConfig {
    min_quality_score: 0.7,
    enable_anomaly_detection: true,
    enable_temporal_consistency: true,
    max_noise_threshold: 0.1,
};

let result = validator.validate_with_config(&sensor_data, &config).await?;

if result.quality_score >= config.min_quality_score {
    println!("Data quality: {:.2}", result.quality_score);
} else {
    println!("Data quality too low: {:.2}", result.quality_score);
}
```

### ROS2 Integration

```rust
use kova_core::ros2::{ROS2Bridge, ROS2Config};

let config = ROS2Config {
    node_name: "kova_bridge".to_string(),
    namespace: "/kova".to_string(),
    qos_profile: QosProfile::default(),
};

let mut bridge = ROS2Bridge::new(config).await?;

// Subscribe to sensor topics
bridge.subscribe_sensor_data("/camera/image_raw").await?;
bridge.subscribe_sensor_data("/lidar/points").await?;
bridge.subscribe_sensor_data("/imu/data").await?;

// Start processing
bridge.start_processing().await?;
```

## Configuration

### Sensor Configuration

```rust
use kova_core::sensors::*;

// Camera configuration
let camera_config = CameraConfig {
    resolution: (1920, 1080),
    frame_rate: 30,
    format: ImageFormat::RGB,
    auto_exposure: true,
    auto_white_balance: true,
    exposure_compensation: 0.0,
    iso_sensitivity: 100,
};

// LiDAR configuration
let lidar_config = LiDARConfig {
    range_min: 0.1,
    range_max: 100.0,
    angular_resolution: 0.1,
    scan_frequency: 10.0,
    point_cloud_format: PointCloudFormat::XYZI,
};

// IMU configuration
let imu_config = IMUConfig {
    sample_rate: 100.0,
    accelerometer_range: AccelerometerRange::G16,
    gyroscope_range: GyroscopeRange::DPS2000,
    magnetometer_enabled: true,
    temperature_compensation: true,
};
```

### Blockchain Configuration

```rust
use kova_core::blockchain::*;

let solana_config = SolanaConfig {
    rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
    commitment: CommitmentLevel::Confirmed,
    timeout: Duration::from_secs(30),
    retry_attempts: 3,
};

let ipfs_config = IPFSConfig {
    api_url: "http://localhost:5001".to_string(),
    gateway_url: "http://localhost:8080".to_string(),
    pin_on_add: true,
    timeout: Duration::from_secs(60),
};
```

## Examples

### Basic Sensor Data Collection

```bash
cargo run --example basic_sensor_collection
```

### ROS2 Integration

```bash
cargo run --example ros2_bridge
```

### Blockchain Integration

```bash
cargo run --example blockchain_integration
```

### Data Validation

```bash
cargo run --example data_validation
```

## Testing

Run all tests:

```bash
cargo test
```

Run specific test categories:

```bash
cargo test --features sensors
cargo test --features blockchain
cargo test --features validation
cargo test --features ros2
```

Run integration tests:

```bash
cargo test --test integration
```

## Building

### Prerequisites

- Rust 1.70+
- Cargo
- CMake (for some dependencies)
- OpenCV (for camera support)
- PCL (for LiDAR support)

### Build Commands

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Build with specific features
cargo build --features "sensors,blockchain,validation,ros2"

# Build documentation
cargo doc --open
```

## Performance

Kova Core is designed for high-performance robotics applications:

- **Low Latency**: Sub-millisecond sensor data processing
- **High Throughput**: Support for multiple sensors at high frame rates
- **Memory Efficient**: Zero-copy data processing where possible
- **Async Processing**: Non-blocking I/O for all operations

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

1. Fork the repository
2. Clone your fork
3. Create a feature branch
4. Make your changes
5. Add tests
6. Run the test suite
7. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Links

- [Website](https://www.kova.systems/)
- [Documentation](https://docs.rs/kova-core)
- [Discord](https://discord.gg/kova)
- [Twitter](https://twitter.com/KovaSystems)

## Acknowledgments

- The Rust community for excellent tooling and ecosystem
- The ROS2 community for robotics middleware
- The Solana team for blockchain infrastructure
- The IPFS team for decentralized storage
- The Kova Systems team for the PoSC protocol

---

**Made with ❤️ by the Kova Systems team**