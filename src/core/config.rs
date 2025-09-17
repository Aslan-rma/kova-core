//! Configuration management for Kova Core

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Sensor configuration
    pub sensors: SensorConfig,
    /// Blockchain configuration
    pub blockchain: BlockchainConfig,
    /// Network configuration
    pub network: NetworkConfig,
    /// Validation configuration
    pub validation: ValidationConfig,
    /// Storage configuration
    pub storage: StorageConfig,
    /// ROS2 configuration
    #[cfg(feature = "ros2")]
    pub ros2: ROS2Config,
}

/// Sensor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorConfig {
    /// Default sensor timeout in seconds
    pub timeout_seconds: u64,
    /// Enable sensor data caching
    pub enable_caching: bool,
    /// Cache size limit in MB
    pub cache_size_mb: usize,
    /// Enable sensor calibration
    pub enable_calibration: bool,
}

/// Blockchain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    /// Solana configuration
    pub solana: SolanaConfig,
    /// IPFS configuration
    pub ipfs: IPFSConfig,
    /// Arweave configuration
    pub arweave: ArweaveConfig,
}

/// Solana configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaConfig {
    /// RPC endpoint URL
    pub rpc_url: String,
    /// Commitment level
    pub commitment: String,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Number of retry attempts
    pub retry_attempts: u32,
}

/// IPFS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPFSConfig {
    /// API endpoint URL
    pub api_url: String,
    /// Gateway URL
    pub gateway_url: String,
    /// Pin content on add
    pub pin_on_add: bool,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
}

/// Arweave configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArweaveConfig {
    /// Gateway URL
    pub gateway_url: String,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Number of retry attempts
    pub retry_attempts: u32,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Connection timeout in seconds
    pub connection_timeout_seconds: u64,
    /// Enable connection pooling
    pub enable_pooling: bool,
    /// Pool size
    pub pool_size: usize,
}

/// Validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Minimum quality score threshold
    pub min_quality_score: f64,
    /// Enable anomaly detection
    pub enable_anomaly_detection: bool,
    /// Enable temporal consistency checks
    pub enable_temporal_consistency: bool,
    /// Maximum noise threshold
    pub max_noise_threshold: f64,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Local storage path
    pub local_path: String,
    /// Enable compression
    pub enable_compression: bool,
    /// Compression level (1-9)
    pub compression_level: u32,
    /// Maximum file size in MB
    pub max_file_size_mb: usize,
}

/// ROS2 configuration
#[cfg(feature = "ros2")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ROS2Config {
    /// Node name
    pub node_name: String,
    /// Namespace
    pub namespace: String,
    /// QoS profile
    pub qos_profile: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sensors: SensorConfig::default(),
            blockchain: BlockchainConfig::default(),
            network: NetworkConfig::default(),
            validation: ValidationConfig::default(),
            storage: StorageConfig::default(),
            #[cfg(feature = "ros2")]
            ros2: ROS2Config::default(),
        }
    }
}

impl Default for SensorConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            enable_caching: true,
            cache_size_mb: 100,
            enable_calibration: true,
        }
    }
}

impl Default for BlockchainConfig {
    fn default() -> Self {
        Self {
            solana: SolanaConfig::default(),
            ipfs: IPFSConfig::default(),
            arweave: ArweaveConfig::default(),
        }
    }
}

impl Default for SolanaConfig {
    fn default() -> Self {
        Self {
            rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            commitment: "confirmed".to_string(),
            timeout_seconds: 30,
            retry_attempts: 3,
        }
    }
}

impl Default for IPFSConfig {
    fn default() -> Self {
        Self {
            api_url: "http://localhost:5001".to_string(),
            gateway_url: "http://localhost:8080".to_string(),
            pin_on_add: true,
            timeout_seconds: 60,
        }
    }
}

impl Default for ArweaveConfig {
    fn default() -> Self {
        Self {
            gateway_url: "https://arweave.net".to_string(),
            timeout_seconds: 30,
            retry_attempts: 3,
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            max_connections: 100,
            connection_timeout_seconds: 30,
            enable_pooling: true,
            pool_size: 10,
        }
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            min_quality_score: 0.7,
            enable_anomaly_detection: true,
            enable_temporal_consistency: true,
            max_noise_threshold: 0.1,
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            local_path: "./data".to_string(),
            enable_compression: true,
            compression_level: 6,
            max_file_size_mb: 100,
        }
    }
}

#[cfg(feature = "ros2")]
impl Default for ROS2Config {
    fn default() -> Self {
        Self {
            node_name: "kova_bridge".to_string(),
            namespace: "/kova".to_string(),
            qos_profile: "default".to_string(),
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
