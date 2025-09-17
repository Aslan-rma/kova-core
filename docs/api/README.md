# Kova Core API Documentation

This directory contains comprehensive API documentation for the Kova Core library.

## Table of Contents

- [Overview](overview.md) - General API overview and concepts
- [Sensors](sensors.md) - Sensor API documentation
- [Blockchain](blockchain.md) - Blockchain integration API
- [Validation](validation.md) - Data validation API
- [Robots](robots.md) - Robot integration API
- [SDK](sdk.md) - SDK generation and usage

## Quick Start

The Kova Core API is designed to be intuitive and easy to use. Here's a basic example:

```rust
use kova_core::{init, SensorManager, BlockchainManager, DataValidator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the core system
    init().await?;
    
    // Create managers
    let mut sensor_manager = SensorManager::new();
    let mut blockchain_manager = BlockchainManager::new();
    let validator = DataValidator::new();
    
    // Your robotics application code here
    
    Ok(())
}
```

## API Reference

### Core Modules

- **Sensors**: Multi-sensor data processing and management
- **Blockchain**: Integration with Solana, IPFS, and Arweave
- **Validation**: Data quality assessment and validation
- **Robots**: Arduino, Raspberry Pi, and ROS2 integration
- **SDK**: Code generation for Python, JavaScript, and Rust

### Error Handling

All API functions return `Result<T, Error>` where `Error` is the main error type:

```rust
use kova_core::core::Error;

match result {
    Ok(data) => println!("Success: {:?}", data),
    Err(Error::Sensor(msg)) => println!("Sensor error: {}", msg),
    Err(Error::Blockchain(msg)) => println!("Blockchain error: {}", msg),
    Err(Error::Validation(msg)) => println!("Validation error: {}", msg),
    Err(e) => println!("Other error: {}", e),
}
```

### Configuration

Most components can be configured using configuration structs:

```rust
use kova_core::sensors::CameraConfig;

let config = CameraConfig {
    resolution: (1920, 1080),
    frame_rate: 30,
    format: ImageFormat::RGB,
    auto_exposure: true,
    auto_white_balance: true,
    exposure_compensation: 0.0,
    iso_sensitivity: 100,
    focus_mode: FocusMode::Auto,
    white_balance_mode: WhiteBalanceMode::Auto,
};
```

## Examples

See the `examples/` directory for comprehensive examples:

- [Basic Robot](examples/basic_robot/) - Simple robot setup
- [Advanced Fleet](examples/advanced_fleet/) - Multi-robot fleet management
- [Agriculture](examples/agriculture/) - Agricultural robotics application

## Contributing

When adding new API features, please:

1. Update the relevant documentation files
2. Add comprehensive examples
3. Include unit tests
4. Update the changelog

## Support

For API questions and support:

- [GitHub Issues](https://github.com/kovasystems/kova-core/issues)
- [Discord](https://discord.gg/kova)
- [Documentation](https://docs.kova.systems/)
