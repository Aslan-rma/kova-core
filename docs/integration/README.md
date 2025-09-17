# Kova Core Integration Guide

This guide covers integrating Kova Core with various robotics platforms and systems.

## Table of Contents

- [ROS2 Integration](ros2.md) - ROS2 bridge setup and usage
- [Arduino Integration](arduino.md) - Arduino robot integration
- [Raspberry Pi Integration](raspberry_pi.md) - Raspberry Pi setup
- [Blockchain Integration](blockchain.md) - Blockchain network setup
- [Sensor Integration](sensors.md) - Sensor hardware integration
- [Cloud Integration](cloud.md) - Cloud platform integration

## Quick Start

### 1. Install Dependencies

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install system dependencies
sudo apt-get update
sudo apt-get install -y build-essential cmake pkg-config
```

### 2. Add to Your Project

```toml
[dependencies]
kova-core = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### 3. Basic Integration

```rust
use kova_core::{init, SensorManager, BlockchainManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Kova Core
    init().await?;
    
    // Create managers
    let mut sensor_manager = SensorManager::new();
    let mut blockchain_manager = BlockchainManager::new();
    
    // Your integration code here
    
    Ok(())
}
```

## Platform-Specific Integration

### ROS2 Integration

```bash
# Install ROS2 dependencies
sudo apt-get install -y ros-humble-desktop

# Add to your ROS2 workspace
cd your_ros2_workspace/src
git clone https://github.com/kovasystems/kova-core.git
colcon build --packages-select kova_core
```

### Arduino Integration

```cpp
// Arduino sketch example
#include <Wire.h>
#include <SPI.h>

void setup() {
    Serial.begin(9600);
    // Initialize sensors
}

void loop() {
    // Read sensor data
    // Send to Kova Core via serial
}
```

### Raspberry Pi Integration

```bash
# Enable I2C and SPI
sudo raspi-config

# Install Python dependencies
pip install kova-core

# Run Python integration
python3 kova_integration.py
```

## Configuration

### Environment Variables

```bash
export KOVA_API_ENDPOINT="http://localhost:8080"
export KOVA_BLOCKCHAIN_ENDPOINT="https://api.mainnet-beta.solana.com"
export KOVA_IPFS_ENDPOINT="http://localhost:5001"
export KOVA_LOG_LEVEL="info"
```

### Configuration Files

Create `kova_config.toml`:

```toml
[sensors]
timeout_seconds = 30
enable_caching = true
cache_size_mb = 100

[blockchain]
solana_rpc_url = "https://api.mainnet-beta.solana.com"
ipfs_api_url = "http://localhost:5001"
arweave_gateway_url = "https://arweave.net"

[validation]
min_quality_score = 0.7
enable_anomaly_detection = true
enable_temporal_consistency = true
```

## Troubleshooting

### Common Issues

1. **Sensor Connection Issues**
   - Check hardware connections
   - Verify sensor drivers
   - Check permissions

2. **Blockchain Connection Issues**
   - Verify network connectivity
   - Check API endpoints
   - Verify credentials

3. **Validation Failures**
   - Check data quality
   - Adjust validation thresholds
   - Review sensor calibration

### Debug Mode

Enable debug logging:

```rust
use tracing_subscriber;

tracing_subscriber::fmt()
    .with_env_filter("kova_core=debug")
    .init();
```

### Performance Optimization

1. **Enable Caching**
   ```rust
   let config = SensorConfig {
       enable_caching: true,
       cache_size_mb: 1000,
       // ...
   };
   ```

2. **Use Async Processing**
   ```rust
   let sensor_data = sensor_manager.capture_all().await?;
   ```

3. **Batch Operations**
   ```rust
   let results = blockchain_manager.submit_batch(contributions).await?;
   ```

## Examples

See the `examples/` directory for platform-specific examples:

- [Basic Robot](examples/basic_robot/) - Simple robot setup
- [Advanced Fleet](examples/advanced_fleet/) - Multi-robot management
- [Agriculture](examples/agriculture/) - Agricultural robotics

## Support

For integration support:

- [GitHub Issues](https://github.com/kovasystems/kova-core/issues)
- [Discord](https://discord.gg/kova)
- [Documentation](https://docs.kova.systems/)
