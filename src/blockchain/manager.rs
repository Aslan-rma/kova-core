//! Blockchain manager for handling multiple blockchain clients

use crate::core::Error;
use crate::blockchain::{BlockchainClient, Contribution};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Blockchain manager for handling multiple blockchain clients
pub struct BlockchainManager {
    clients: RwLock<HashMap<String, Box<dyn BlockchainClient>>>,
}

impl BlockchainManager {
    /// Create a new blockchain manager
    pub fn new() -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
        }
    }

    /// Add a blockchain client
    pub async fn add_client(&self, name: String, client: Box<dyn BlockchainClient>) {
        let mut clients = self.clients.write().await;
        clients.insert(name, client);
    }

    /// Store data using the first available client
    pub async fn store_data(&self, data: &[u8]) -> Result<String, Error> {
        let clients = self.clients.read().await;
        
        for (name, client) in clients.iter() {
            if client.is_available().await {
                match client.store_data(data).await {
                    Ok(hash) => {
                        tracing::info!("Data stored using {}: {}", name, hash);
                        return Ok(hash);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to store data using {}: {}", name, e);
                    }
                }
            }
        }
        
        Err(Error::blockchain("No available blockchain clients"))
    }

    /// Retrieve data using the first available client
    pub async fn retrieve_data(&self, hash: &str) -> Result<Vec<u8>, Error> {
        let clients = self.clients.read().await;
        
        for (name, client) in clients.iter() {
            if client.is_available().await {
                match client.retrieve_data(hash).await {
                    Ok(data) => {
                        tracing::info!("Data retrieved using {}: {} bytes", name, data.len());
                        return Ok(data);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to retrieve data using {}: {}", name, e);
                    }
                }
            }
        }
        
        Err(Error::blockchain("No available blockchain clients"))
    }

    /// Submit a contribution
    pub async fn submit_contribution(&self, contribution: &Contribution) -> Result<String, Error> {
        // Serialize contribution
        let data = serde_json::to_vec(contribution)?;
        
        // Store on blockchain
        let hash = self.store_data(&data).await?;
        
        tracing::info!("Contribution submitted: {}", hash);
        Ok(hash)
    }
}
