//! IPFS blockchain integration

use crate::core::Error;
use crate::blockchain::{BlockchainClient, Contribution};
use serde::{Deserialize, Serialize};

/// IPFS client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPFSConfig {
    /// API endpoint URL
    pub api_url: String,
    /// Gateway URL
    pub gateway_url: String,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Number of retry attempts
    pub retry_attempts: u32,
    /// Pin content on add
    pub pin_on_add: bool,
}

/// IPFS client implementation
pub struct IPFSClient {
    config: IPFSConfig,
    client: reqwest::Client,
}

impl IPFSClient {
    /// Create a new IPFS client
    pub async fn new(config: IPFSConfig) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| Error::blockchain(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { config, client })
    }

    /// Add data to IPFS
    pub async fn add_data(&self, data: &[u8]) -> Result<String, Error> {
        let form = reqwest::multipart::Form::new()
            .part("file", reqwest::multipart::Part::bytes(data.to_vec()));

        let response = self.client
            .post(&format!("{}/api/v0/add", self.config.api_url))
            .multipart(form)
            .send()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to add data to IPFS: {}", e)))?;

        let response_text = response
            .text()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to read response: {}", e)))?;

        // Parse IPFS response
        let lines: Vec<&str> = response_text.trim().split('\n').collect();
        let last_line = lines.last()
            .ok_or_else(|| Error::blockchain("Empty IPFS response"))?;

        let response_json: serde_json::Value = serde_json::from_str(last_line)
            .map_err(|e| Error::blockchain(format!("Failed to parse IPFS response: {}", e)))?;

        let hash = response_json["Hash"]
            .as_str()
            .ok_or_else(|| Error::blockchain("No hash in IPFS response"))?;

        // Pin the content if configured
        if self.config.pin_on_add {
            self.pin(hash).await?;
        }

        Ok(hash.to_string())
    }

    /// Get data from IPFS
    pub async fn get_data(&self, hash: &str) -> Result<Vec<u8>, Error> {
        let url = format!("{}/api/v0/cat?arg={}", self.config.api_url, hash);
        
        let response = self.client
            .post(&url)
            .send()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to get data from IPFS: {}", e)))?;

        let data = response
            .bytes()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to read data: {}", e)))?;

        Ok(data.to_vec())
    }

    /// Pin content in IPFS
    pub async fn pin(&self, hash: &str) -> Result<(), Error> {
        let url = format!("{}/api/v0/pin/add?arg={}", self.config.api_url, hash);
        
        self.client
            .post(&url)
            .send()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to pin content: {}", e)))?;

        Ok(())
    }

    /// Unpin content from IPFS
    pub async fn unpin(&self, hash: &str) -> Result<(), Error> {
        let url = format!("{}/api/v0/pin/rm?arg={}", self.config.api_url, hash);
        
        self.client
            .post(&url)
            .send()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to unpin content: {}", e)))?;

        Ok(())
    }

    /// Get pinned content list
    pub async fn list_pins(&self) -> Result<Vec<String>, Error> {
        let url = format!("{}/api/v0/pin/ls", self.config.api_url);
        
        let response = self.client
            .post(&url)
            .send()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to list pins: {}", e)))?;

        let response_text = response
            .text()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to read response: {}", e)))?;

        let response_json: serde_json::Value = serde_json::from_str(&response_text)
            .map_err(|e| Error::blockchain(format!("Failed to parse response: {}", e)))?;

        let pins = response_json["Keys"]
            .as_object()
            .ok_or_else(|| Error::blockchain("Invalid pins response"))?;

        let mut pin_hashes = Vec::new();
        for (hash, _) in pins {
            pin_hashes.push(hash.clone());
        }

        Ok(pin_hashes)
    }

    /// Get content statistics
    pub async fn get_stats(&self, hash: &str) -> Result<IPFSStats, Error> {
        let url = format!("{}/api/v0/object/stat?arg={}", self.config.api_url, hash);
        
        let response = self.client
            .post(&url)
            .send()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to get stats: {}", e)))?;

        let response_text = response
            .text()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to read response: {}", e)))?;

        let stats: IPFSStats = serde_json::from_str(&response_text)
            .map_err(|e| Error::blockchain(format!("Failed to parse stats: {}", e)))?;

        Ok(stats)
    }

    /// Check if IPFS node is available
    pub async fn check_availability(&self) -> Result<bool, Error> {
        let url = format!("{}/api/v0/version", self.config.api_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

/// IPFS statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPFSStats {
    pub hash: String,
    pub num_links: u32,
    pub block_size: u32,
    pub links_size: u32,
    pub data_size: u32,
    pub cumulative_size: u32,
}

impl Default for IPFSConfig {
    fn default() -> Self {
        Self {
            api_url: "http://localhost:5001".to_string(),
            gateway_url: "http://localhost:8080".to_string(),
            timeout_seconds: 60,
            retry_attempts: 3,
            pin_on_add: true,
        }
    }
}

impl BlockchainClient for IPFSClient {
    fn name(&self) -> &str {
        "IPFS"
    }
    
    async fn is_available(&self) -> bool {
        match self.check_availability().await {
            Ok(available) => available,
            Err(_) => false,
        }
    }
    
    async fn store_data(&self, data: &[u8]) -> Result<String, Error> {
        self.add_data(data).await
    }
    
    async fn retrieve_data(&self, hash: &str) -> Result<Vec<u8>, Error> {
        self.get_data(hash).await
    }
}