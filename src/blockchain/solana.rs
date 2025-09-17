//! Solana blockchain integration

use crate::core::Error;
use crate::blockchain::{BlockchainClient, Contribution};
use serde::{Deserialize, Serialize};

/// Solana client configuration
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
    /// Private key for signing transactions
    pub private_key: Option<String>,
}

/// Solana client implementation
pub struct SolanaClient {
    config: SolanaConfig,
    client: reqwest::Client,
}

impl SolanaClient {
    /// Create a new Solana client
    pub async fn new(config: SolanaConfig) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| Error::blockchain(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { config, client })
    }

    /// Get account balance
    pub async fn get_balance(&self, address: &str) -> Result<f64, Error> {
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getBalance",
            "params": [address]
        });

        let response = self.client
            .post(&self.config.rpc_url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to send request: {}", e)))?;

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to parse response: {}", e)))?;

        let balance = response_json["result"]["value"]
            .as_u64()
            .ok_or_else(|| Error::blockchain("Invalid balance response"))?;

        // Convert lamports to SOL (1 SOL = 1,000,000,000 lamports)
        Ok(balance as f64 / 1_000_000_000.0)
    }

    /// Submit a transaction
    pub async fn submit_transaction(&self, transaction: &str) -> Result<String, Error> {
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sendTransaction",
            "params": [transaction]
        });

        let response = self.client
            .post(&self.config.rpc_url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to send transaction: {}", e)))?;

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to parse response: {}", e)))?;

        let signature = response_json["result"]
            .as_str()
            .ok_or_else(|| Error::blockchain("Invalid transaction response"))?;

        Ok(signature.to_string())
    }

    /// Get transaction status
    pub async fn get_transaction_status(&self, signature: &str) -> Result<bool, Error> {
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getSignatureStatuses",
            "params": [[signature]]
        });

        let response = self.client
            .post(&self.config.rpc_url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to get transaction status: {}", e)))?;

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::blockchain(format!("Failed to parse response: {}", e)))?;

        let status = response_json["result"]["value"][0]["confirmationStatus"]
            .as_str()
            .unwrap_or("unknown");

        Ok(status == "confirmed" || status == "finalized")
    }

    /// Create a contribution transaction
    pub async fn create_contribution_transaction(&self, contribution: &Contribution) -> Result<String, Error> {
        // Serialize contribution data
        let contribution_data = serde_json::to_vec(contribution)
            .map_err(|e| Error::blockchain(format!("Failed to serialize contribution: {}", e)))?;

        // Create transaction (simplified)
        let transaction = base64::encode(&contribution_data);
        Ok(transaction)
    }
}

impl Default for SolanaConfig {
    fn default() -> Self {
        Self {
            rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            commitment: "confirmed".to_string(),
            timeout_seconds: 30,
            retry_attempts: 3,
            private_key: None,
        }
    }
}

impl BlockchainClient for SolanaClient {
    fn name(&self) -> &str {
        "Solana"
    }
    
    async fn is_available(&self) -> bool {
        match self.get_balance("11111111111111111111111111111112").await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    
    async fn store_data(&self, data: &[u8]) -> Result<String, Error> {
        // For Solana, we would typically store data in a program account
        // This is a simplified implementation
        let hash = sha2::Sha256::digest(data);
        let hash_hex = hex::encode(hash);
        
        // In a real implementation, this would create a transaction
        // to store the data in a Solana program account
        Ok(format!("solana:{}", hash_hex))
    }
    
    async fn retrieve_data(&self, hash: &str) -> Result<Vec<u8>, Error> {
        // For Solana, we would retrieve data from a program account
        // This is a simplified implementation
        if hash.starts_with("solana:") {
            let actual_hash = &hash[7..];
            // In a real implementation, this would query the Solana program
            // to retrieve the stored data
            Err(Error::blockchain("Data retrieval not implemented"))
        } else {
            Err(Error::blockchain("Invalid Solana hash format"))
        }
    }
}