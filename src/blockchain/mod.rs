//! Blockchain integration for Kova Core

pub mod arweave;
pub mod ipfs;
pub mod solana;
pub mod manager;

pub use manager::BlockchainManager;

/// Blockchain client trait
pub trait BlockchainClient: Send + Sync {
    /// Get client name
    fn name(&self) -> &str;
    
    /// Check if client is available
    async fn is_available(&self) -> bool;
    
    /// Store data
    async fn store_data(&self, data: &[u8]) -> Result<String, crate::core::Error>;
    
    /// Retrieve data
    async fn retrieve_data(&self, hash: &str) -> Result<Vec<u8>, crate::core::Error>;
}

/// Contribution data structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Contribution {
    /// Sensor data hash
    pub sensor_data_hash: String,
    /// Validator signature
    pub validator_signature: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Quality score
    pub quality_score: f64,
    /// Validator ID
    pub validator_id: String,
    /// Sensor ID
    pub sensor_id: String,
}