//! Arweave blockchain integration

use crate::core::Error;
use crate::blockchain::{BlockchainClient, Contribution};
use serde::{Deserialize, Serialize};

/// Arweave client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArweaveConfig {
    /// Gateway URL
    pub gateway_url: String,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Number of retry attempts
    pub retry_attempts: u32,
    /// Private key for signing transactions
    pub private_key: Option<String>,
}

/// Arweave client implementation
pub struct ArweaveClient {
    config: ArweaveConfig,
    client: reqwest::Client,
}

impl ArweaveClient {
    /// Create a new Arweave client
    pub async fn new(config: ArweaveConfig) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| Error::blockchain(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { config, client })
    }

    /// Upload data to Arweave
    pub async fn upload_data(&self, data: &[u8], tags: Vec<(String, String)>) -> Result<String, Error> {
        let transaction = self.create_transaction(data, tags).await?;
        let transaction_id = self.submit_transaction(&transaction).await?;
        Ok(transaction_id)
    }

    /// Create a transaction
    async fn create_transaction(&self, data: &[u8], tags: Vec<(String, String)>) -> Result<ArweaveTransaction, Error> {
        // Generate transaction ID (simplified)
        let transaction_id = self.generate_transaction_id(data);
        
        // Calculate data size
        let data_size = data.len() as u64;
        
        // Calculate reward (simplified)
        let reward = self.calculate_reward(data_size);
        
        // Create transaction
        let transaction = ArweaveTransaction {
            format: 2,
            id: transaction_id,
            last_tx: "".to_string(), // Would be fetched from network
            owner: "".to_string(), // Would be derived from private key
            tags: tags.into_iter().map(|(name, value)| ArweaveTag { name, value }).collect(),
            target: "".to_string(),
            quantity: "0".to_string(),
            data: base64::encode(data),
            reward: reward.to_string(),
        };

        Ok(transaction)
    }

    /// Submit transaction to Arweave
    async fn submit_transaction(&self, transaction: &ArweaveTransaction) -> Result<String, Error> {
        let url = format!("{}/tx", self.config.gateway_url);
        
        let response = self.client
            .post(&url)
            .json(transaction)
            .send()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to submit transaction: {}", e)))?;

        if response.status().is_success() {
            Ok(transaction.id.clone())
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(Error::blockchain(format!("Transaction failed: {}", error_text)))
        }
    }

    /// Get data from Arweave
    pub async fn get_data(&self, transaction_id: &str) -> Result<Vec<u8>, Error> {
        let url = format!("{}/{}", self.config.gateway_url, transaction_id);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to get data: {}", e)))?;

        let data = response
            .bytes()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to read data: {}", e)))?;

        Ok(data.to_vec())
    }

    /// Get transaction information
    pub async fn get_transaction_info(&self, transaction_id: &str) -> Result<ArweaveTransactionInfo, Error> {
        let url = format!("{}/tx/{}/status", self.config.gateway_url, transaction_id);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to get transaction info: {}", e)))?;

        let info: ArweaveTransactionInfo = response
            .json()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to parse transaction info: {}", e)))?;

        Ok(info)
    }

    /// Generate transaction ID
    fn generate_transaction_id(&self, data: &[u8]) -> String {
        let hash = sha2::Sha256::digest(data);
        base64::encode(hash)
    }

    /// Calculate transaction reward
    fn calculate_reward(&self, data_size: u64) -> u64 {
        // Simplified reward calculation
        // In reality, this would query the network for current price
        data_size * 1000 // 1000 winston per byte
    }

    /// Check if Arweave node is available
    pub async fn check_availability(&self) -> Result<bool, Error> {
        let url = format!("{}/info", self.config.gateway_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

/// Arweave transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArweaveTransaction {
    pub format: u32,
    pub id: String,
    pub last_tx: String,
    pub owner: String,
    pub tags: Vec<ArweaveTag>,
    pub target: String,
    pub quantity: String,
    pub data: String,
    pub reward: String,
}

/// Arweave tag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArweaveTag {
    pub name: String,
    pub value: String,
}

/// Arweave transaction information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArweaveTransactionInfo {
    pub block_height: u64,
    pub block_indep_hash: String,
    pub number_of_confirmations: u64,
}

impl Default for ArweaveConfig {
    fn default() -> Self {
        Self {
            gateway_url: "https://arweave.net".to_string(),
            timeout_seconds: 30,
            retry_attempts: 3,
            private_key: None,
        }
    }
}

impl BlockchainClient for ArweaveClient {
    fn name(&self) -> &str {
        "Arweave"
    }
    
    async fn is_available(&self) -> bool {
        match self.check_availability().await {
            Ok(available) => available,
            Err(_) => false,
        }
    }
    
    async fn store_data(&self, data: &[u8]) -> Result<String, Error> {
        let tags = vec![
            ("Content-Type".to_string(), "application/octet-stream".to_string()),
            ("App-Name".to_string(), "Kova".to_string()),
        ];
        self.upload_data(data, tags).await
    }
    
    async fn retrieve_data(&self, hash: &str) -> Result<Vec<u8>, Error> {
        self.get_data(hash).await
    }
}
